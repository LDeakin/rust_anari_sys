# anari_sys

[![Latest Version](https://img.shields.io/crates/v/anari_sys.svg)](https://crates.io/crates/anari_sys)
[![anari_sys documentation](https://docs.rs/anari_sys/badge.svg)](https://docs.rs/anari_sys)
[![ANARI 1.0](https://img.shields.io/badge/ANARI-1.0-blue)](https://www.khronos.org/registry/ANARI/)
[![ANARI-SDK 0.12.1](https://img.shields.io/badge/ANARI--SDK-0.12.1-blue)](https://github.com/KhronosGroup/ANARI-SDK)
[![build](https://github.com/LDeakin/rust_anari_sys/actions/workflows/ci.yml/badge.svg)](https://github.com/LDeakin/rust_anari_sys/actions/workflows/ci.yml)

Raw Rust bindings to the ANARI frontend library (<https://github.com/KhronosGroup/ANARI-SDK>), a 3D rendering engine interface API.

## Bindings
This library includes a pre-generated `bindings.rs` file. New bindings can be generated using the bindgen feature:
```bash
cargo build --features bindgen
```

## Example
```bash
# with the libanari_library_helide dynamic library findable (e.g. in LD_LIBRARY_PATH on linux)
cargo run --example info helide
```

## Licence
`anari_sys` is licensed under the Apache License, Version 2.0 [LICENSE-APACHE](./LICENCE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
