use lscpu::Cpu;

fn main() {
    let cpu = Cpu::new();
    println!("{}", cpu);
}