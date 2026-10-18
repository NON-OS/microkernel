// NONOS Operating System (AGPL-3.0-or-later)
//! The move to the high half's rewrite: every pointer moves by exactly the
//! high alias's distance, nothing else changes, and a refused rebase writes
//! nothing, so the kernel keeps running where it was.

use crate::elf::parse;
use crate::loaded::Loaded;
use crate::relocate::rela::Refused;

const PIE: &[u8] = include_bytes!("../fixtures/pie.elf");
/// The kernel's high alias: its direct map's base.
const HIGH: u64 = 0xFFFF_8000_0000_0000;
/// Where the fixture's writable data starts, from its link base.
const WRITABLE: u64 = 0x18_0000;

#[test]
fn every_pointer_moves_by_the_alias_and_nothing_else() {
    let pie = parse(PIE);
    let img = Loaded::new(&pie, PIE);
    img.relocate(&pie).unwrap();
    let before = img.bytes().to_vec();
    assert_eq!(img.rebase(&pie, img.base + HIGH, WRITABLE), Ok(pie.relas.len()));
    let places: Vec<u64> = pie.relas.iter().map(|r| r.offset - pie.link_base).collect();
    for at in &places {
        let was = u64::from_le_bytes(before[*at as usize..*at as usize + 8].try_into().unwrap());
        assert_eq!(img.word(*at), was + HIGH);
    }
    let moved = |i: usize| places.iter().any(|p| (*p as usize..*p as usize + 8).contains(&i));
    let after = img.bytes();
    assert!((0..after.len()).filter(|i| !moved(*i)).all(|i| after[i] == before[i]));
}

/// A pointer before the writable part, or a base off the 2 MiB grid, is
/// refused before anything is written.
#[test]
fn a_refusal_writes_nothing() {
    let pie = parse(PIE);
    let img = Loaded::new(&pie, PIE);
    img.relocate(&pie).unwrap();
    let before = img.bytes().to_vec();
    assert_eq!(img.rebase(&pie, img.base + HIGH, WRITABLE + 8), Err(Refused::ReadOnlyPlace));
    assert_eq!(img.rebase(&pie, img.base + HIGH + 4096, WRITABLE), Err(Refused::LoadMisaligned));
    assert_eq!(img.bytes(), &before[..]);
}
