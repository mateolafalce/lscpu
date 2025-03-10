use lscpu::Cpu;

fn main() {
    let cpu_data = Cpu::new();
    println!("{}", cpu_data);
    let vendor = cpu_data.get_vendor_id();
    println!("{}", vendor);
}
