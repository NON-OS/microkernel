// NONOS Operating System (AGPL-3.0-or-later)
//! Data objects (section 20.2.3).

use super::{cat, pkg};

/// An integer in the shortest encoding: Zero, One, Ones or a prefixed
/// constant.
pub fn int(v: u64) -> Vec<u8> {
    match v {
        0 => vec![0x00],
        1 => vec![0x01],
        u64::MAX => vec![0xFF],
        v if v <= 0xFF => vec![0x0A, v as u8],
        v if v <= 0xFFFF => cat(&[&[0x0B], &(v as u16).to_le_bytes()]),
        v if v <= 0xFFFF_FFFF => cat(&[&[0x0C], &(v as u32).to_le_bytes()]),
        v => cat(&[&[0x0E], &v.to_le_bytes()]),
    }
}

pub fn string(s: &str) -> Vec<u8> {
    cat(&[&[0x0D], s.as_bytes(), &[0x00]])
}

/// Buffer(size) { bytes }.
pub fn buffer(size: u64, bytes: &[u8]) -> Vec<u8> {
    pkg(0x11, &cat(&[&int(size), bytes]))
}

/// Package(count) { elements }, each element already encoded.
pub fn package(count: u8, elements: &[Vec<u8>]) -> Vec<u8> {
    pkg(0x12, &cat(&[&[count], &elements.concat()]))
}

pub const LOCAL0: u8 = 0x60;
pub const ARG0: u8 = 0x68;
/// NullName, as an absent Target.
pub const NULL: u8 = 0x00;
