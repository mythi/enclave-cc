# Notes to Setup SGX VM for CI Testing

# SGX PSW

- install `aesmd`, configure with `default quoting type = ecdsa_256` and the preferred DCAP quote provider lib (with its config)

# containerd
We pre-install the CoCo containerd and tools with a simple command:

```bash
curl -fsSL https://github.com/confidential-containers/containerd/releases/download/v1.6.8.1/cri-containerd-cni-1.6.8.1-linux-amd64.tar.gz | sudo tar zx -C /
```

And it's configured with the following runtimehandler:
```
$ tail -3 /etc/containerd/config.toml
[plugins."io.containerd.grpc.v1.cri".containerd.runtimes.enclavecc]
  cri_handler = "cc"
  runtime_type = "io.containerd.rune.v2"
```

# verdictd (for EAA-KBC testing)

## Set-up

We provide a verdictd image for quick set-up. Suppose a good sgx configuration is prepared in `/etc/sgx_default_qcnl.conf` where a non-localhost PCCS is configured.
Run the following scripts to set up a verdictd server
```bash
mkdir -p $VERDICTD_WORKDIR/data
docker run -d -v /etc/sgx_default_qcnl.conf:/etc/sgx_default_qcnl.conf \
    -v $VERDICTD_WORKDIR/data:/opt/verdictd \
    --device /dev/sgx_enclave \
    --device /dev/sgx_provision \
    -p 12345:12345 \
    --env RUST_LOG=debug \
    xynnn007/verdictd:v0.5.0-rc1 \
    verdictd \
    --listen 0.0.0.0:12345 \
    --verifier sgx_ecdsa \
    --attester nullattester \
    --client-api 127.0.0.1:50000 \
    --mutual
```

This will mount the directory `$VERDICTD_WORKDIR/data` as into the container to work as the data directory, and this service will listen to port `12345`.

## Default Configurations (Optional)
We can put some default configurations as following
```bash
mkdir -p $VERDICTD_WORKDIR/data/resources/default/security-policy

cat <<EOF > $VERDICTD_WORKDIR/data/resources/default/security-policy/test
{
    "default": [{"type": "reject"}], 
    "transports": {
        "docker": {
            "ghcr.io/confidential-containers": [
                {
                    "type": "insecureAcceptAnything"
                }
            ]
        }
    }
}
EOF

mkdir -p $VERDICTD_WORKDIR/data/resources/default/sigstore-config

cat <<EOF > $VERDICTD_WORKDIR/data/resources/default/sigstore-config/test
default:
    sigstore: file:///var/lib/containers/sigstore

EOF

mkdir -p $VERDICTD_WORKDIR/data/opa
mkdir -p $VERDICTD_WORKDIR/data/keys

cat <<EOF > $VERDICTD_WORKDIR/data/opa/sgxData
{
    "mrEnclave": [],
    "mrSigner": [],
    "productId": 0,
    "svn": 0
}
EOF

cat <<EOF > $VERDICTD_WORKDIR/data/opa/sgxPolicy.rego

package policy

# By default, deny requests.
default allow = false

allow {
    mrEnclave_is_grant
    mrSigner_is_grant
    input.productId >= data.productId
    input.svn >= data.svn
}

mrEnclave_is_grant {
    count(data.mrEnclave) == 0
}
mrEnclave_is_grant {
    count(data.mrEnclave) > 0
    input.mrEnclave == data.mrEnclave[_]
}

mrSigner_is_grant {
    count(data.mrSigner) == 0
}
mrSigner_is_grant {
    count(data.mrSigner) > 0
    input.mrSigner == data.mrSigner[_]
}

EOF
```

# KBS (for CC-KBC testing)

We can set up a KBS cluster with docker-compose quickly.
```
git clone https://github.com/confidential-containers/kbs.git && cd kbs
```

1. change the as's image in `docker-compose.yml` to `docker.io/xynnn007/attestation-service:sgx-v0.6.0`
2. change the PCCS configuration volume of as in `docker-compose.yml` to `/etc/sgx_default_qcnl.conf:/etc/sgx_default_qcnl.conf:rw`
3. Run
```bash
openssl genpkey -algorithm ed25519 > config/private.key
openssl pkey -in config/private.key -pubout -out config/public.pub
```
4. Run with `docker compose up -d`, and KBS will listen to port `8080` for requests

We can add default configs under `data/kbs-storage` the same as verdictd.

# Github Runner Service
For `enclave-cc` e2e tests, we run a "job-started" pre-cleanup job configured
for the runner:

```
Environment=ACTIONS_RUNNER_HOOK_JOB_STARTED=<path/to>/job-started-cleanup.sh
```

The script content:
```
#!/bin/bash

echo "delete previous workspace $GITHUB_WORKSPACE"
pushd $GITHUB_WORKSPACE
sudo rm -rf coco src
popd

echo "delete lingering pods"
for i in $(sudo crictl pods -q); do
    sudo crictl -t 10s stopp $i;
    sudo crictl -t 10s rmp $i;
done

echo "docker system prune"
docker system prune -a -f
```
