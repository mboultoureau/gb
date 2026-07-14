fn main() {
    let reg = gb_core::registers::Registers::default();
    println!("Hello, world! AF = 0x{:04X}", reg.af());
}
