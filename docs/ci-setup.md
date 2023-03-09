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
- install [verdictd](https://github.com/inclavare-containers/verdictd) and its dependencies
- [RATS-TLS](https://github.com/inclavare-containers/rats-tls) version is recommended to be the same as what the `agent-enclave` build uses.
- `/usr/local/bin/verdictd --listen 127.0.0.1:1234 --verifier sgx_ecdsa --attester nullattester --client-api 127.0.0.1:12340 --mutual`

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
