// NONOS Operating System (AGPL-3.0-or-later)
//! OperationRegion and field lists in ASL's terms.

use super::{cat, int, name, op, pkg};

pub fn region(path: &str, space: u8, base: u64, len: u64) -> Vec<u8> {
    cat(&[&op(0x5B80), &name(path), &[space], &int(base), &int(len)])
}

/// Field(region, flags) { elements }.
pub fn field(region: &str, flags: u8, elements: &[Vec<u8>]) -> Vec<u8> {
    pkg(0x5B81, &cat(&[&name(region), &[flags], &elements.concat()]))
}

/// A NamedField: four characters, then the width in bits.
pub fn unit(seg: &str, bits: usize) -> Vec<u8> {
    let mut s = [b'_'; 4];
    s[..seg.len()].copy_from_slice(seg.as_bytes());
    cat(&[&s, &bits_len(bits)])
}

pub fn skip(bits: usize) -> Vec<u8> {
    cat(&[&[0x00], &bits_len(bits)])
}

pub fn access_as(ty: u8, attrib: u8) -> Vec<u8> {
    vec![0x01, ty, attrib]
}

/// A field width uses PkgLength's encoding for the bit count itself, with
/// no bytes of its own counted in.
fn bits_len(v: usize) -> Vec<u8> {
    let follow = match v {
        0..=0x3F => return vec![v as u8],
        0x40..=0xFFF => 1,
        0x1000..=0xF_FFFF => 2,
        _ => 3,
    };
    let mut enc = vec![((follow as u8) << 6) | (v & 0x0F) as u8];
    enc.extend((0..follow).map(|i| (v >> (4 + 8 * i)) as u8));
    enc
}
