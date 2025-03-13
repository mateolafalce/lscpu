# lscpu

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
