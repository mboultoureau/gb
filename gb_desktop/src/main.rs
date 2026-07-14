fn main() {
    let reg = gb_core::Registers::default();
    println!("Hello, world! AF = 0x{:04X}", reg.af());
}
