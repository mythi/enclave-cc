FROM ubuntu:22.04 as builder

RUN apt-get update && \
    env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    lsb-release \
    curl \
    ca-certificates \
    gnupg

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN curl -fsSLo /usr/share/keyrings/gramine-keyring.gpg https://packages.gramineproject.io/gramine-keyring.gpg
RUN echo "deb [arch=amd64 signed-by=/usr/share/keyrings/gramine-keyring.gpg] https://packages.gramineproject.io/ $(lsb_release -sc) main" \
| tee /etc/apt/sources.list.d/gramine.list && \
    apt-get update && \
    env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    make \
    binutils \
    build-essential \
    git \
    tzdata \
    protobuf-compiler \
    gramine

RUN git clone -b enclave_cc --depth 1 https://github.com/ying2liu/gramine.git

WORKDIR /gramine/CI-Examples/enclave_cc_sig

COPY tools/packaging/build/gramine-agent-enclave-bundle/build.rs image-rs

RUN cd enclave-cc/src/enclave-agent && \
    . $HOME/.cargo/env && \
    sed -i -e 's:^gramine_feature.*:gramine_feature = ["attestation_agent/default", "ocicrypt-rs/default"]:g' ../../../image-rs/Cargo.toml && \
    cargo build --release

FROM ubuntu:22.04

RUN apt-get update && \
    env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    lsb-release \
    curl \
    ca-certificates \
    gnupg

RUN curl -fsSLo /usr/share/keyrings/gramine-keyring.gpg https://packages.gramineproject.io/gramine-keyring.gpg
RUN echo "deb [arch=amd64 signed-by=/usr/share/keyrings/gramine-keyring.gpg] https://packages.gramineproject.io/ $(lsb_release -sc) main" \
| tee /etc/apt/sources.list.d/gramine.list && \
    apt-get update && \
    env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    gramine

COPY --from=builder /gramine/CI-Examples/enclave_cc_sig/enclave-cc/src/enclave-agent/target/release/enclave-agent .
COPY tools/packaging/build/gramine-agent-enclave-bundle/enclave-agent.manifest.template .
RUN gramine-sgx-gen-private-key && \
    gramine-manifest \
    -Dlog_level=debug \
    -Darch_libdir=/lib/x86_64-linux-gnu \
    enclave-agent.manifest.template >enclave-agent.manifest && \
    gramine-sgx-sign \
    --manifest enclave-agent.manifest \
    --output enclave-agent.manifest.sgx
