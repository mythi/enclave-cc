// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

fn main() -> shadow_rs::SdResult<()> {
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=static=sgx_util");
    println!("cargo:rustc-link-lib=static=mbedcrypto_gramine");
    tonic_build::compile_protos("./protos/getresource.proto")?;

    shadow_rs::new()
}
