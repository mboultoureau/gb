use core::fmt;

const HEADER_END: usize = 0x014F;
const MIN_ROM_LEN: usize = HEADER_END + 1;

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderError {
    TooSmall(usize),
    InvalidLogo,
    UnknownCartridgeType(u8),
    UnknownRomSize(u8),
    UnknownRamSize(u8),
    HeaderChecksumMismatch { stored: u8, computed: u8 },
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooSmall(len) => {
                write!(
                    f,
                    "ROM is too small to contain a header: {len} < {MIN_ROM_LEN} bytes"
                )
            }
            Self::InvalidLogo => f.write_str("Nintendo logo does not match the expected bitmap"),
            Self::UnknownCartridgeType(byte) => write!(f, "unknown cartridge type: {byte:#04X}"),
            Self::UnknownRomSize(byte) => write!(f, "unknown ROM size code: {byte:#04X}"),
            Self::UnknownRamSize(byte) => write!(f, "unknown RAM size code: {byte:#04X}"),
            Self::HeaderChecksumMismatch { stored, computed } => write!(
                f,
                "header checksum mismatch: stored {stored:#04X}, computed {computed:#04X}"
            ),
        }
    }
}

impl core::error::Error for HeaderError {}

/// Game Boy Color support flag (`$0143`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum CgbFlag {
    MonochromeOnly(u8),
    Enhanced,
    Exclusive,
}

impl fmt::Display for CgbFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MonochromeOnly(byte) => write!(f, "monochrome only ({byte:#04X})"),
            Self::Enhanced => f.write_str("CGB enhanced (DMG compatible)"),
            Self::Exclusive => f.write_str("CGB only"),
        }
    }
}

impl From<u8> for CgbFlag {
    fn from(byte: u8) -> Self {
        match byte {
            0x80 => Self::Enhanced,
            0xC0 => Self::Exclusive,
            other => Self::MonochromeOnly(other),
        }
    }
}

/// Super Game Boy support flag (`$0146`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum SgbFlag {
    Unsupported(u8),
    Supported,
}

impl fmt::Display for SgbFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(byte) => write!(f, "unsupported ({byte:#04X})"),
            Self::Supported => f.write_str("supported"),
        }
    }
}

impl From<u8> for SgbFlag {
    fn from(byte: u8) -> Self {
        match byte {
            0x03 => Self::Supported,
            other => Self::Unsupported(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
}

impl CartridgeType {
    #[must_use]
    pub fn has_battery(self) -> bool {
        matches!(
            self,
            Self::Mbc1RamBattery
                | Self::Mbc2Battery
                | Self::RomRamBattery
                | Self::Mmm01RamBattery
                | Self::Mbc3TimerBattery
                | Self::Mbc3TimerRamBattery
                | Self::Mbc3RamBattery
                | Self::Mbc5RamBattery
                | Self::Mbc5RumbleRamBattery
                | Self::Mbc7SensorRumbleRamBattery
                | Self::HuC1RamBattery
        )
    }
}

impl TryFrom<u8> for CartridgeType {
    type Error = HeaderError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        let ty = match byte {
            0x00 => Self::RomOnly,
            0x01 => Self::Mbc1,
            0x02 => Self::Mbc1Ram,
            0x03 => Self::Mbc1RamBattery,
            0x05 => Self::Mbc2,
            0x06 => Self::Mbc2Battery,
            0x08 => Self::RomRam,
            0x09 => Self::RomRamBattery,
            0x0B => Self::Mmm01,
            0x0C => Self::Mmm01Ram,
            0x0D => Self::Mmm01RamBattery,
            0x0F => Self::Mbc3TimerBattery,
            0x10 => Self::Mbc3TimerRamBattery,
            0x11 => Self::Mbc3,
            0x12 => Self::Mbc3Ram,
            0x13 => Self::Mbc3RamBattery,
            0x19 => Self::Mbc5,
            0x1A => Self::Mbc5Ram,
            0x1B => Self::Mbc5RamBattery,
            0x1C => Self::Mbc5Rumble,
            0x1D => Self::Mbc5RumbleRam,
            0x1E => Self::Mbc5RumbleRamBattery,
            0x20 => Self::Mbc6,
            0x22 => Self::Mbc7SensorRumbleRamBattery,
            0xFC => Self::PocketCamera,
            0xFD => Self::BandaiTama5,
            0xFE => Self::HuC3,
            0xFF => Self::HuC1RamBattery,
            other => return Err(HeaderError::UnknownCartridgeType(other)),
        };
        Ok(ty)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RomSize {
    bytes: usize,
    banks: usize,
}

impl RomSize {
    #[must_use]
    pub fn bytes(self) -> usize {
        self.bytes
    }

    #[must_use]
    pub fn banks(self) -> usize {
        self.banks
    }
}

impl TryFrom<u8> for RomSize {
    type Error = HeaderError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        let banks = match byte {
            0x00..=0x08 => 2usize << byte,
            other => return Err(HeaderError::UnknownRomSize(other)),
        };
        Ok(Self {
            banks,
            bytes: banks * 16 * 1024,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RamSize {
    bytes: usize,
    banks: usize,
}

impl RamSize {
    #[must_use]
    pub fn bytes(self) -> usize {
        self.bytes
    }

    #[must_use]
    pub fn banks(self) -> usize {
        self.banks
    }
}

impl TryFrom<u8> for RamSize {
    type Error = HeaderError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        let banks = match byte {
            0x00 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            other => return Err(HeaderError::UnknownRamSize(other)),
        };
        Ok(Self {
            banks,
            bytes: banks * 8 * 1024,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum DestinationCode {
    Japan,
    Overseas,
    Unknown(u8),
}

impl fmt::Display for DestinationCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Japan => f.write_str("Japan (and possibly overseas)"),
            Self::Overseas => f.write_str("Overseas only"),
            Self::Unknown(byte) => write!(f, "unknown ({byte:#04X})"),
        }
    }
}

impl From<u8> for DestinationCode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::Japan,
            0x01 => Self::Overseas,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Header {
    pub entrypoint: [u8; 4],
    pub title: [u8; 16],
    pub cgb_flag: CgbFlag,
    pub new_licensee_code: [u8; 2],
    pub sgb_flag: SgbFlag,
    pub cartridge_type: CartridgeType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub destination_code: DestinationCode,
    pub old_licensee_code: u8,
    pub version: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

impl Header {
    #[must_use]
    pub fn compute_header_checksum(rom: &[u8]) -> u8 {
        rom[0x0134..=0x014C]
            .iter()
            .fold(0u8, |acc, &byte| acc.wrapping_sub(byte).wrapping_sub(1))
    }

    #[must_use]
    pub fn validate_nintendo_logo(logo: &[u8; 48]) -> bool {
        *logo == NINTENDO_LOGO
    }

    pub fn read(rom: &[u8]) -> Result<Self, HeaderError> {
        if rom.len() < MIN_ROM_LEN {
            return Err(HeaderError::TooSmall(rom.len()));
        }

        // Constant ranges guarantee the slice lengths, so every `expect` below is unreachable.
        let logo: [u8; 48] = rom[0x0104..=0x0133].try_into().expect("48 bytes");
        if !Self::validate_nintendo_logo(&logo) {
            return Err(HeaderError::InvalidLogo);
        }

        let header_checksum = rom[0x014D];
        let computed = Self::compute_header_checksum(rom);
        if header_checksum != computed {
            return Err(HeaderError::HeaderChecksumMismatch {
                stored: header_checksum,
                computed,
            });
        }

        let entrypoint: [u8; 4] = rom[0x0100..=0x0103].try_into().expect("4 bytes");
        let title: [u8; 16] = rom[0x0134..=0x0143].try_into().expect("16 bytes");
        let new_licensee_code: [u8; 2] = rom[0x0144..=0x0145].try_into().expect("2 bytes");

        Ok(Self {
            entrypoint,
            title,
            cgb_flag: CgbFlag::from(rom[0x0143]),
            new_licensee_code,
            sgb_flag: SgbFlag::from(rom[0x0146]),
            cartridge_type: CartridgeType::try_from(rom[0x0147])?,
            rom_size: RomSize::try_from(rom[0x0148])?,
            ram_size: RamSize::try_from(rom[0x0149])?,
            destination_code: DestinationCode::from(rom[0x014A]),
            old_licensee_code: rom[0x014B],
            version: rom[0x014C],
            header_checksum,
            global_checksum: u16::from_be_bytes([rom[0x014E], rom[0x014F]]),
        })
    }

    #[must_use]
    pub fn title_str(&self) -> Option<&str> {
        let end = self
            .title
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.title.len());
        core::str::from_utf8(&self.title[..end]).ok()
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Title:            {}",
            self.title_str().unwrap_or("<invalid UTF-8>")
        )?;
        writeln!(f, "Cartridge type:   {:?}", self.cartridge_type)?;
        writeln!(
            f,
            "ROM size:         {} KiB ({} banks)",
            self.rom_size.bytes() / 1024,
            self.rom_size.banks()
        )?;
        writeln!(
            f,
            "RAM size:         {} KiB ({} banks)",
            self.ram_size.bytes() / 1024,
            self.ram_size.banks()
        )?;
        writeln!(f, "Battery:          {}", self.cartridge_type.has_battery())?;
        writeln!(f, "CGB flag:         {}", self.cgb_flag)?;
        writeln!(f, "SGB flag:         {}", self.sgb_flag)?;
        writeln!(f, "Destination:      {}", self.destination_code)?;
        writeln!(f, "New licensee:     {:?}", self.new_licensee_code)?;
        writeln!(f, "Old licensee:     {:#04X}", self.old_licensee_code)?;
        writeln!(f, "Version:          {}", self.version)?;
        writeln!(f, "Header checksum:  {:#04X}", self.header_checksum)?;
        write!(f, "Global checksum:  {:#06X}", self.global_checksum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rom() -> Vec<u8> {
        let mut rom = vec![0u8; MIN_ROM_LEN];

        rom[0x0100..=0x0103].copy_from_slice(&[0x00, 0xC3, 0x50, 0x01]);
        rom[0x0104..=0x0133].copy_from_slice(&NINTENDO_LOGO);
        rom[0x0134..0x0138].copy_from_slice(b"TEST");
        rom[0x0144..=0x0145].copy_from_slice(b"01");
        rom[0x0146] = 0x00;
        rom[0x0147] = 0x00;
        rom[0x0148] = 0x00;
        rom[0x0149] = 0x00;
        rom[0x014A] = 0x01;
        rom[0x014B] = 0x33;
        rom[0x014C] = 0x00;

        rom[0x014D] = Header::compute_header_checksum(&rom);
        rom[0x014E] = 0x12;
        rom[0x014F] = 0x34;

        rom
    }

    #[test]
    fn reads_a_valid_header() {
        let rom = make_rom();
        let header = Header::read(&rom).expect("valid header");

        assert_eq!(header.entrypoint, [0x00, 0xC3, 0x50, 0x01]);
        assert_eq!(header.title_str(), Some("TEST"));
        assert_eq!(header.new_licensee_code, *b"01");
        assert_eq!(header.sgb_flag, SgbFlag::Unsupported(0x00));
        assert_eq!(header.cartridge_type, CartridgeType::RomOnly);
        assert_eq!(header.rom_size.bytes(), 32 * 1024);
        assert_eq!(header.rom_size.banks(), 2);
        assert_eq!(header.ram_size.bytes(), 0);
        assert_eq!(header.ram_size.banks(), 0);
        assert_eq!(header.destination_code, DestinationCode::Overseas);
        assert_eq!(header.old_licensee_code, 0x33);
        assert_eq!(header.version, 0);
        assert_eq!(header.global_checksum, 0x1234);
    }

    #[test]
    fn rejects_a_rom_that_is_too_small() {
        let rom = [0u8; 16];
        assert_eq!(Header::read(&rom), Err(HeaderError::TooSmall(16)));
    }

    #[test]
    fn rejects_an_invalid_logo() {
        let mut rom = make_rom();
        rom[0x0104] ^= 0xFF;
        assert_eq!(Header::read(&rom), Err(HeaderError::InvalidLogo));
    }

    #[test]
    fn rejects_a_bad_header_checksum() {
        let mut rom = make_rom();
        let good = rom[0x014D];
        rom[0x014D] = good.wrapping_add(1);
        assert_eq!(
            Header::read(&rom),
            Err(HeaderError::HeaderChecksumMismatch {
                stored: good.wrapping_add(1),
                computed: good,
            })
        );
    }

    #[test]
    fn rejects_an_unknown_cartridge_type() {
        let mut rom = make_rom();
        rom[0x0147] = 0x04;
        rom[0x014D] = Header::compute_header_checksum(&rom);
        assert_eq!(
            Header::read(&rom),
            Err(HeaderError::UnknownCartridgeType(0x04))
        );
    }

    #[test]
    fn rejects_an_unknown_rom_size() {
        let mut rom = make_rom();
        rom[0x0148] = 0x52;
        rom[0x014D] = Header::compute_header_checksum(&rom);
        assert_eq!(Header::read(&rom), Err(HeaderError::UnknownRomSize(0x52)));
    }

    #[test]
    fn rejects_an_unknown_ram_size() {
        let mut rom = make_rom();
        rom[0x0149] = 0x01;
        rom[0x014D] = Header::compute_header_checksum(&rom);
        assert_eq!(Header::read(&rom), Err(HeaderError::UnknownRamSize(0x01)));
    }

    #[test]
    fn cgb_flag_conversions() {
        assert_eq!(CgbFlag::from(0x80), CgbFlag::Enhanced);
        assert_eq!(CgbFlag::from(0xC0), CgbFlag::Exclusive);
        assert_eq!(CgbFlag::from(0x00), CgbFlag::MonochromeOnly(0x00));
    }

    #[test]
    fn cartridge_type_battery_detection() {
        assert!(CartridgeType::Mbc3RamBattery.has_battery());
        assert!(!CartridgeType::RomOnly.has_battery());
        assert!(!CartridgeType::Mbc1.has_battery());
    }

    #[test]
    fn rom_and_ram_size_scaling() {
        assert_eq!(RomSize::try_from(0x01).unwrap().bytes(), 64 * 1024);
        assert_eq!(RomSize::try_from(0x05).unwrap().banks(), 64);
        assert_eq!(RamSize::try_from(0x03).unwrap().bytes(), 32 * 1024);
        assert_eq!(RamSize::try_from(0x03).unwrap().banks(), 4);
    }

    #[test]
    fn validates_the_reference_logo() {
        assert!(Header::validate_nintendo_logo(&NINTENDO_LOGO));
        let mut wrong = NINTENDO_LOGO;
        wrong[0] = 0x00;
        assert!(!Header::validate_nintendo_logo(&wrong));
    }
}
