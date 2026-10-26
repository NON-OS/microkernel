// NONOS Operating System (AGPL-3.0-or-later)
//! The AML the `_DSM` sweep builds from (dsm_gen.rs): the HID-over-I2C
//! GUID, a seeded generator, PkgLength-headed terms and integers in each
//! encoding AML has.

use alloc::vec::Vec;

pub const GUID: [u8; 16] = [
    0xF7, 0xF6, 0xDF, 0x3C, 0x67, 0x42, 0x55, 0x45, 0xAD, 0x05, 0xB3, 0x0A, 0x3D, 0x89, 0x38, 0xDE,
];

pub struct Rng(pub u64);

impl Rng {
    pub fn below(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n
    }
}

/// A PkgLength and its content (one byte under 64, else two).
pub fn pkg(c: &[u8]) -> Vec<u8> {
    let n = c.len();
    let head = if n + 1 < 64 {
        alloc::vec![(n + 1) as u8]
    } else {
        alloc::vec![0x40 | ((n + 2) & 0xF) as u8, ((n + 2) >> 4) as u8]
    };
    [head, c.to_vec()].concat()
}

pub fn op(code: &[u8], c: &[u8]) -> Vec<u8> {
    [code, &pkg(c)].concat()
}

/// The integer `v` as AML writes it, and what it means.
pub fn integer(r: &mut Rng, v: u64) -> Vec<u8> {
    match (v, r.below(2)) {
        (0, 0) => alloc::vec![0x00],
        (1, 0) => alloc::vec![0x01],
        (0..=0xFF, _) => alloc::vec![0x0A, v as u8],
        (0..=0xFFFF, _) => [&[0x0B][..], &(v as u16).to_le_bytes()].concat(),
        _ => [&[0x0C][..], &(v as u32).to_le_bytes()].concat(),
    }
}
