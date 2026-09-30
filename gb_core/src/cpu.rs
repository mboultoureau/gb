#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Nop,
    Unknown(u8)
}

pub struct Cpu {

}

impl Cpu {
    pub fn read_opcode(opcode: u8) -> Instruction {
        match opcode {
            0b0000_0000 => Instruction::Nop,
            _ => Instruction::Unknown(opcode)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_opcode() {
        let nop_instr = Cpu::read_opcode(0b0000_0000);
        assert_eq!(nop_instr, Instruction::Nop);

        let unknown_instr = Cpu::read_opcode(0xFF);
        assert_eq!(unknown_instr, Instruction::Unknown(0xFF));
    }
}
