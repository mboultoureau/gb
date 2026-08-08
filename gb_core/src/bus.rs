use crate::cartridge::Cartridge;

const WRAM_SIZE: usize = 8 * 1024;
const HRAM_SIZE: usize = 127;
const IO_SIZE: usize = 128;

/// The memory bus: routes 16-bit CPU addresses to memory-mapped components.
///
/// PanDocs <https://gbdev.io/pandocs/Memory_Map.html>
pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    io: [u8; IO_SIZE],
    ie: u8,
}

impl Bus {
    /// Inserts a cartridge; all RAM starts zeroed.
    #[must_use]
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            io: [0; IO_SIZE],
            ie: 0,
        }
    }

    /// Reads the byte at `addr`; unmapped regions return `0xFF`.
    #[must_use]
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cartridge.read(addr),
            0x8000..=0x9FFF => 0xFF,
            0xA000..=0xBFFF => 0xFF,
            0xC000..=0xDFFF => self.wram[usize::from(addr - 0xC000)],
            0xE000..=0xFDFF => self.wram[usize::from(addr - 0xE000)],
            0xFE00..=0xFE9F => 0xFF,
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00..=0xFF7F => self.io[usize::from(addr - 0xFF00)],
            0xFF80..=0xFFFE => self.hram[usize::from(addr - 0xFF80)],
            0xFFFF => self.ie,
        }
    }

    /// Writes `value` at `addr`; writes to unmapped regions are ignored.
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xC000..=0xDFFF => self.wram[usize::from(addr - 0xC000)] = value,
            0xE000..=0xFDFF => self.wram[usize::from(addr - 0xE000)] = value,
            0xFF00..=0xFF7F => self.io[usize::from(addr - 0xFF00)] = value,
            0xFF80..=0xFFFE => self.hram[usize::from(addr - 0xFF80)] = value,
            0xFFFF => self.ie = value,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::make_rom;

    fn make_bus() -> Bus {
        let mut rom = make_rom();
        rom.resize(0x8000, 0);
        Bus::new(Cartridge::from_rom(rom).expect("valid test ROM"))
    }

    #[test]
    fn reads_rom_through_the_cartridge() {
        let bus = make_bus();
        assert_eq!(bus.read(0x0000), 0x00);
        assert_eq!(bus.read(0x0104), 0xCE);
        assert_eq!(bus.read(0x0134), b'T');
        assert_eq!(bus.read(0x7FFF), 0x00);
    }

    #[test]
    fn writes_to_rom_are_ignored() {
        let mut bus = make_bus();
        bus.write(0x0000, 0x42);
        bus.write(0x7FFF, 0x42);
        assert_eq!(bus.read(0x0000), 0x00);
        assert_eq!(bus.read(0x7FFF), 0x00);
    }

    #[test]
    fn wram_reads_back_written_values() {
        let mut bus = make_bus();
        for addr in [0xC000, 0xC123, 0xDFFF] {
            bus.write(addr, 0xAB);
            assert_eq!(bus.read(addr), 0xAB);
        }
    }

    #[test]
    fn echo_ram_mirrors_wram() {
        let mut bus = make_bus();
        bus.write(0xC000, 0x12);
        assert_eq!(bus.read(0xE000), 0x12);

        bus.write(0xDDFF, 0x34);
        assert_eq!(bus.read(0xFDFF), 0x34);

        bus.write(0xE000, 0x56);
        assert_eq!(bus.read(0xC000), 0x56);
    }

    #[test]
    fn hram_reads_back_written_values() {
        let mut bus = make_bus();
        bus.write(0xFF80, 0x77);
        bus.write(0xFFFE, 0x88);
        assert_eq!(bus.read(0xFF80), 0x77);
        assert_eq!(bus.read(0xFFFE), 0x88);
    }

    #[test]
    fn io_registers_read_back_written_values() {
        let mut bus = make_bus();
        for (addr, value) in [(0xFF00, 0x20), (0xFF04, 0x99), (0xFF7F, 0x55)] {
            bus.write(addr, value);
            assert_eq!(bus.read(addr), value);
        }
    }

    #[test]
    fn ie_register_reads_back_written_value() {
        let mut bus = make_bus();
        assert_eq!(bus.read(0xFFFF), 0x00);
        bus.write(0xFFFF, 0x1F);
        assert_eq!(bus.read(0xFFFF), 0x1F);
    }

    #[test]
    fn unmapped_regions_read_ff_and_ignore_writes() {
        let mut bus = make_bus();
        for addr in [
            0x8000, 0x9FFF, 0xA000, 0xBFFF, 0xFE00, 0xFE9F, 0xFEA0, 0xFEFF,
        ] {
            assert_eq!(bus.read(addr), 0xFF);
            bus.write(addr, 0x00);
            assert_eq!(bus.read(addr), 0xFF);
        }
    }
}
