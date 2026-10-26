// NONOS Operating System (AGPL-3.0-or-later)
//! The device tree header, the arm64 Image header and the ID register
//! fields, each against its specification's layout.

use crate::fdt_header::fdt_size;
use crate::id_fields::IdRegs;
use crate::image_header::image_size;

fn fdt(magic: u32, size: u32) -> [u8; 8] {
    let mut h = [0u8; 8];
    h[..4].copy_from_slice(&magic.to_be_bytes());
    h[4..].copy_from_slice(&size.to_be_bytes());
    h
}

#[test]
fn a_tree_is_its_big_endian_magic_and_size() {
    assert_eq!(fdt_size(fdt(0xd00d_feed, 0x10_0000)), Some(0x10_0000));
    assert_eq!(fdt_size(fdt(0xd00d_feed, 40)), Some(40));
    assert_eq!(fdt_size(fdt(0xd00d_feed, 39)), None, "smaller than its own header");
    assert_eq!(fdt_size(fdt(0xedfe_0dd0, 0x1000)), None, "magic little endian");
    let mut le = fdt(0xd00d_feed, 0);
    le[4..].copy_from_slice(&0x1000u32.to_le_bytes());
    assert_eq!(fdt_size(le), Some(0x0010_0000), "the size is read big endian");
}

fn image(size: u64, magic: &[u8; 4]) -> [u8; 64] {
    let mut h = [0u8; 64];
    h[16..24].copy_from_slice(&size.to_le_bytes());
    h[56..60].copy_from_slice(magic);
    h
}

#[test]
fn an_image_claims_its_size_only_with_the_magic() {
    assert_eq!(image_size(&image(0x579_0000, b"ARM\x64")), Some(0x579_0000));
    assert_eq!(image_size(&image(0x579_0000, b"ARM\x65")), None);
    assert_eq!(image_size(&image(0, b"ARM\x64")), None, "a zero size claims nothing");
    assert_eq!(image_size(&image(1, b"ARM\x64")[..59]), None, "short of the magic");
    let mut h = image(0, b"ARM\x64");
    h[23] = 0x01;
    assert_eq!(image_size(&h), Some(1 << 56), "the size is all eight bytes, little endian");
}

#[test]
fn the_physical_address_size_follows_parange() {
    let bits = |pa: u64| IdRegs { mmfr0: 0xF0 | pa, ..IdRegs::default() }.physical_bits();
    let want = [32, 36, 40, 42, 44, 48, 52, 56];
    for (pa, &w) in want.iter().enumerate() {
        assert_eq!(bits(pa as u64), w, "PARange {pa}");
    }
    for pa in 8..16 {
        assert_eq!(bits(pa), 0, "PARange {pa} is not defined");
    }
}

#[test]
fn each_feature_is_its_own_field() {
    let none = IdRegs::default();
    assert!(!none.pan() && !none.rndr() && !none.aes() && !none.sha2());
    let r = |isar0, mmfr1| IdRegs { isar0, mmfr1, ..IdRegs::default() };
    assert!(r(0, 1 << 20).pan() && !r(!(0xF << 20), !(0xF << 20)).pan());
    assert!(r(1 << 60, 0).rndr() && !r(!(0xF << 60), 0).rndr());
    assert!(r(2 << 4, 0).aes() && !r(!(0xF << 4), 0).aes());
    assert!(r(1 << 12, 0).sha2() && !r(!(0xF << 12), 0).sha2());
    let va = |v: u64| IdRegs { mmfr2: v << 16, ..IdRegs::default() }.virtual_bits();
    assert_eq!((va(0), va(1), va(2)), (48, 52, 48));
}
