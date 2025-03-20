<div align="center">

# lscpu

[<img alt="crates.io" src="https://img.shields.io/crates/v/lscpu.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/lscpu)
[<img alt="github" src="https://img.shields.io/badge/gitlab-mateolafalce/lscpu-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mateolafalce/lscpu)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-lscpu-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/lscpu)

</div>

Implementation of [lscpu](https://www.man7.org/linux/man-pages/man1/lscpu.1.html) in rust

## Run a std example

```
cargo run --example std
```

This code can be runned also in a `no-std` environment.

## Cpu Data:

```rust
pub struct Cpu {
    pub architecture: &'static str,
    pub cpu_op_modes: &'static str,
    pub address_sizes: String,
    pub byte_order: &'static str,
    pub cpu_count: u32,
    pub on_line_cpu: u32,
    pub vendor_id: String,
    pub model_name: String,
    pub cpu_family: u32,
    pub cpu_model: u32,
    pub is_hybrid: &'static str,
    pub threads_per_core: u32,
    pub cores_per_socket: u32,
    pub sockets: u32,
    pub stepping: u32,
    pub boost_enabled: &'static str,
}
```
