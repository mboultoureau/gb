use gb_core::Header;
use gb_core::cartridge::{CartridgeType, DestinationCode, SgbFlag};

// dmg-acid2 by Matt Currie — an MIT-licensed PPU test ROM made for emulator
// developers, so it is safe to commit and redistribute.
// Source: https://github.com/mattcurrie/dmg-acid2
// License: roms/test/dmg-acid2.LICENSE
const DMG_ACID2: &[u8] = include_bytes!("../../roms/test/dmg-acid2.gb");

#[test]
fn parses_dmg_acid2_header() {
    let header = Header::read(DMG_ACID2).expect("dmg-acid2 has a valid header");

    assert_eq!(header.entrypoint, [0x00, 0xC3, 0x50, 0x01]);
    assert_eq!(header.title_str(), Some("DMG-ACID2"));
    assert_eq!(header.cartridge_type, CartridgeType::RomOnly);
    assert!(!header.cartridge_type.has_battery());
    assert_eq!(header.rom_size.bytes(), 32 * 1024);
    assert_eq!(header.ram_size.bytes(), 0);
    assert_eq!(header.sgb_flag, SgbFlag::Unsupported(0x00));
    assert_eq!(header.destination_code, DestinationCode::Japan);
    assert_eq!(header.old_licensee_code, 0x00);
    assert_eq!(
        header.header_checksum,
        Header::compute_header_checksum(DMG_ACID2)
    );
}
