# lscpu

Implementation of [lscpu](https://www.man7.org/linux/man-pages/man1/lscpu.1.html) in rust

## Run a std example

```
cargo run --example std
```

This code can be runned also in a `no-std` environment but you will need to figure out how to print the String :)

## Preview

```rust
use lscpu::lscpu;

fn main() {
    println!("{}", lscpu());
}
```