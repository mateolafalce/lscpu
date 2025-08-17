<div align="center">

# lscpu

[<img alt="crates.io" src="https://img.shields.io/crates/v/lscpu.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/lscpu)
[<img alt="github" src="https://img.shields.io/badge/github-mateolafalce/lscpu-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mateolafalce/lscpu)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-lscpu-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/lscpu)

A Rust implementation of the [lscpu](https://www.man7.org/linux/man-pages/man1/lscpu.1.html) command-line utility for displaying CPU architecture information.

</div>

## Features

- Written in pure Rust
- Fast and lightweight
- `no_std` compatible
- Easy to integrate as a library
- Cross-platform support (Linux focus)
- Zero external dependencies

## Installation

### As a Binary

Install the command-line tool directly from crates.io:

```bash
cargo install lscpu
```

After installation, you can run:

```bash
lscpu
```

### As a Library

Add this to your `Cargo.toml`:

```toml
cargo add lscpu
```

## Usage

### Command Line

Simply run the installed binary to get CPU information:

```bash
$ lscpu
Architecture:             x86_64
CPU op-mode(s):           32-bit, 64-bit
Address sizes:            48 bits physical, 48 bits virtual
Byte Order:               Little Endian
CPU(s):                   2
On-line CPU(s) list:      0,1
Vendor ID:                AuthenticAMD
Model name:               AMD A4-4000 APU with Radeon(tm) HD Graphics    
CPU family:               21
Model:                    19
Is hybrid:                no
Thread(s) per core:       2
Core(s) per socket:       1
Socket(s):                1
Stepping:                 1
Frequency boost:          enabled

            
mateo@debian:~/dev/lscpu$ cargo run
   Compiling lscpu v1.0.8 (/home/mateo/dev/lscpu)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/lscpu`
Architecture:             x86_64
CPU op-mode(s):           32-bit, 64-bit
Address sizes:            48 bits physical, 48 bits virtual
Byte Order:               Little Endian
CPU(s):                   2
On-line CPU(s) list:      0,1
Vendor ID:                AuthenticAMD
Model name:               AMD A4-4000 APU with Radeon(tm) HD Graphics    
CPU family:               21
Model:                    19
Is hybrid:                no
Thread(s) per core:       2
Core(s) per socket:       1
Socket(s):                1
Stepping:                 1
Frequency boost:          enabled
```

### As a Library

```rust
use lscpu::Cpu;

fn main() {
    let cpu = Cpu::new();
    
    println!("Architecture: {}", cpu.architecture);
    println!("Model name: {}", cpu.model_name);
    println!("CPU count: {}", cpu.cpu_count);
    println!("Vendor ID: {}", cpu.vendor_id);
}
```

## Development

### Building from Source

```bash
git clone https://github.com/mateolafalce/lscpu.git
cd lscpu
cargo build --release
```

### Running Examples

Run the standard example:

```bash
cargo run --example std
```

This demonstrates basic usage and can also be run in a `no-std` environment.

## CPU Data Structure

The main `Cpu` struct provides comprehensive CPU information:

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

## Platform Support

Currently supports:
- Linux (primary target)
- Other Unix-like systems (limited support)
- Windows (not supported yet)

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original `lscpu` utility from util-linux
- Built with ❤️ in Rust
