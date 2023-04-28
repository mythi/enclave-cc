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

RUN git clone -b enclave_cc_seal_key --depth 1 https://github.com/ying2liu/gramine.git

WORKDIR /gramine/CI-Examples/enclave_cc_seal_key_new

COPY tools/packaging/build/gramine-agent-enclave-bundle/build.rs image-rs
COPY tools/packaging/build/gramine-agent-enclave-bundle/Cargo.toml enclave-cc/src/enclave-agent

RUN cd enclave-cc/src/enclave-agent && \
    . $HOME/.cargo/env && \
    sed -i -e 's:^gramine_feature.*:gramine_feature = ["attestation_agent/default", "ocicrypt-rs/default"]:g' ../../../image-rs/Cargo.toml && \
    cargo build --release --no-default-features --features simple

FROM ubuntu:22.04

RUN apt-get update && \
    env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    lsb-release \
    curl \
    ca-certificates \
    gnupg

RUN curl -fsSL https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | apt-key add -
RUN echo "deb [arch=amd64] https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -sc) main" | tee /etc/apt/sources.list.d/intel-sgx.list && \
    apt-get update && \
    env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    protobuf-compiler \
    autoconf \
    git \
    bison \
    gawk \
    build-essential \
    nasm \
    ninja-build \
    pkg-config \
    python3 \
    python3-click \
    python3-jinja2 \
    python3-pip \
    python3-pyelftools \
    libsgx-dcap-quote-verify-dev \
    protobuf-c-compiler \
    protobuf-compiler \
    libprotobuf-c-dev \
    python3-cryptography \
    python3-pip \
    python3-protobuf \
    wget && \
    python3 -m pip install 'meson>=0.56' 'tomli>=1.1.0' 'tomli-w>=0.4.0'

RUN git clone --depth 1 https://github.com/gramineproject/gramine.git
RUN cd gramine && \
    meson setup build/ --buildtype=release -Ddirect=enabled -Dsgx=enabled && \
    ninja -C build/ && \
    ninja -C build/ install
RUN rm -rf gramine

COPY --from=builder /gramine/CI-Examples/enclave_cc_seal_key_new/enclave-cc/src/enclave-agent/target/release/enclave-agent .
COPY tools/packaging/build/gramine-agent-enclave-bundle/enclave-agent.manifest.template .
RUN gramine-sgx-gen-private-key && \
    gramine-manifest \
    -Dlog_level=debug \
    -Darch_libdir=/lib/x86_64-linux-gnu \
    enclave-agent.manifest.template >enclave-agent.manifest && \
    gramine-sgx-sign \
    --manifest enclave-agent.manifest \
    --output enclave-agent.manifest.sgx
