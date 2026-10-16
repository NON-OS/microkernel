// NONOS Operating System (AGPL-3.0-or-later)

//! Malformed firmware files are refused whole.

use super::images::SO_GF;
use crate::gen3::ucode::Ucode;

// A minimal well-formed file: header, then the given records.
fn file(records: &[(u32, &[u8])]) -> Vec<u8> {
    let mut f = vec![0u8; 88];
    f[4..8].copy_from_slice(&0x0A4C_5749u32.to_le_bytes());
    for (ty, body) in records {
        f.extend_from_slice(&ty.to_le_bytes());
        f.extend_from_slice(&(body.len() as u32).to_le_bytes());
        f.extend_from_slice(body);
        while !f.len().is_multiple_of(4) {
            f.push(0);
        }
    }
    f
}

#[test]
fn malformed_firmware_files_are_refused_whole() {
    let iml: &[u8] = &[1, 2, 3, 4];
    assert!(Ucode::parse(&file(&[(52, iml)])).is_some(), "the minimal file parses");
    assert!(Ucode::parse(&file(&[])).is_none(), "no image loader");
    let mut bad_magic = file(&[(52, iml)]);
    bad_magic[4] ^= 1;
    assert!(Ucode::parse(&bad_magic).is_none(), "wrong magic");
    assert!(Ucode::parse(&bad_magic[..40]).is_none(), "short header");
    let long_phy: &[u8] = &[0; 8];
    assert!(
        Ucode::parse(&file(&[(52, iml), (23, long_phy)])).is_none(),
        "PHY_SKU of the wrong size"
    );
    let mut past_end = file(&[(52, iml)]);
    past_end.extend_from_slice(&23u32.to_le_bytes());
    past_end.extend_from_slice(&64u32.to_le_bytes());
    assert!(Ucode::parse(&past_end).is_none(), "a record running past the end");
    let real = SO_GF;
    for cut in [88, 100, 4096, real.len() / 2, real.len() - 3] {
        let _ = Ucode::parse(&real[..cut]);
    }
}

#[test]
fn bitmap_words_past_the_known_range_are_ignored_and_partial_entries_dropped() {
    let iml: &[u8] = &[9; 4];
    let far: &[u8] = &[9, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF];
    let near: &[u8] = &[3, 0, 0, 0, 0, 0x40, 0, 0];
    let table: &[u8] = &[0x0D, 1, 17, 0, 0x0C, 1];
    let blob = file(&[(52, iml), (30, far), (30, near), (48, table)]);
    let u = Ucode::parse(&blob).expect("parses");
    assert!(u.capa(110), "word 3 bit 14");
    assert!(!u.capa(300), "nothing past 128 bits");
    assert_eq!(u.cmd_version(1, 0x0D), Some(17));
    assert_eq!(u.cmd_version(1, 0x0C), None, "the half entry is not read");
}
