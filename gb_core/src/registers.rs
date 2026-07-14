// PanDocs <https://gbdev.io/pandocs/CPU_Registers_and_Flags.html>

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub const FLAG_Z: u8 = 1 << 7;
    pub const FLAG_N: u8 = 1 << 6;
    pub const FLAG_H: u8 = 1 << 5;
    pub const FLAG_C: u8 = 1 << 4;

    #[inline]
    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f])
    }

    #[inline]
    pub fn set_af(&mut self, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.a = high;
        self.f = low & 0xF0;
    }

    #[inline]
    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    #[inline]
    pub fn set_bc(&mut self, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.b = high;
        self.c = low;
    }

    #[inline]
    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    #[inline]
    pub fn set_de(&mut self, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.d = high;
        self.e = low;
    }

    #[inline]
    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    #[inline]
    pub fn set_hl(&mut self, value: u16) {
        let [high, low] = value.to_be_bytes();
        self.h = high;
        self.l = low;
    }

    #[inline]
    pub fn flag(&self, mask: u8) -> bool {
        self.f & mask != 0
    }

    #[inline]
    pub fn set_flag(&mut self, mask: u8, value: bool) {
        if value {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }
    }

    pub fn zero(&self) -> bool {
        self.flag(Self::FLAG_Z)
    }
    pub fn subtract(&self) -> bool {
        self.flag(Self::FLAG_N)
    }
    pub fn half_carry(&self) -> bool {
        self.flag(Self::FLAG_H)
    }
    pub fn carry(&self) -> bool {
        self.flag(Self::FLAG_C)
    }

    pub fn set_zero(&mut self, v: bool) {
        self.set_flag(Self::FLAG_Z, v)
    }
    pub fn set_subtract(&mut self, v: bool) {
        self.set_flag(Self::FLAG_N, v)
    }
    pub fn set_half_carry(&mut self, v: bool) {
        self.set_flag(Self::FLAG_H, v)
    }
    pub fn set_carry(&mut self, v: bool) {
        self.set_flag(Self::FLAG_C, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_pairs() {
        let mut reg = Registers::default();

        reg.set_bc(0x1234);
        assert_eq!(reg.b, 0x12);
        assert_eq!(reg.c, 0x34);
        assert_eq!(reg.bc(), 0x1234);

        reg.set_de(0x5678);
        assert_eq!(reg.d, 0x56);
        assert_eq!(reg.e, 0x78);
        assert_eq!(reg.de(), 0x5678);

        reg.set_hl(0x9ABC);
        assert_eq!(reg.h, 0x9A);
        assert_eq!(reg.l, 0xBC);
        assert_eq!(reg.hl(), 0x9ABC);
    }

    #[test]
    fn test_af_masking() {
        let mut reg = Registers::default();

        reg.set_af(0xFFFF);
        assert_eq!(reg.a, 0xFF);
        assert_eq!(reg.f, 0xF0);
        assert_eq!(reg.af(), 0xFFF0);

        reg.set_af(0x000F);
        assert_eq!(reg.f, 0x00);
        assert_eq!(reg.af(), 0x0000);
    }

    #[test]
    fn test_flag_toggles() {
        let mut reg = Registers::default();

        reg.set_zero(true);
        assert!(reg.zero());
        assert_eq!(reg.f, Registers::FLAG_Z);

        reg.set_zero(false);
        assert!(!reg.zero());
        assert_eq!(reg.f, 0);

        reg.set_subtract(true);
        assert!(reg.subtract());
        assert_eq!(reg.f, Registers::FLAG_N);

        reg.set_subtract(false);
        assert!(!reg.subtract());
        assert_eq!(reg.f, 0);

        reg.set_half_carry(true);
        assert!(reg.half_carry());
        assert_eq!(reg.f, Registers::FLAG_H);

        reg.set_half_carry(false);
        assert!(!reg.half_carry());
        assert_eq!(reg.f, 0);

        reg.set_carry(true);
        assert!(reg.carry());
        assert_eq!(reg.f, Registers::FLAG_C);

        reg.set_carry(false);
        assert!(!reg.carry());
        assert_eq!(reg.f, 0);
    }

    #[test]
    fn test_sp_and_pc() {
        let reg = Registers {
            sp: 0xFFFE,
            pc: 0x0100,
            ..Default::default()
        };
        assert_eq!(reg.sp, 0xFFFE);
        assert_eq!(reg.pc, 0x0100);
    }

    #[test]
    fn test_all_flags_combined() {
        let mut reg = Registers::default();

        reg.set_zero(true);
        reg.set_subtract(true);
        reg.set_half_carry(true);
        reg.set_carry(true);
        assert_eq!(reg.f, 0xF0);
        assert_eq!(reg.af(), 0x00F0);

        reg.set_af(0xFFFF);
        assert!(reg.zero());
        assert!(reg.subtract());
        assert!(reg.half_carry());
        assert!(reg.carry());
    }

    #[test]
    fn test_flag_independence() {
        let mut reg = Registers::default();

        reg.set_zero(true);
        reg.set_carry(true);
        assert_eq!(reg.f, Registers::FLAG_Z | Registers::FLAG_C);

        reg.set_zero(false);
        assert_eq!(reg.f, Registers::FLAG_C);

        reg.set_subtract(true);
        assert_eq!(reg.f, Registers::FLAG_N | Registers::FLAG_C);
    }
}
