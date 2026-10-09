//! The three bytes at the end of an address, as five symbols.
//!
//! Twenty four bits do not fill five symbols. The twenty fifth bit is padding, written
//! as zero, and reading refuses a set padding bit, so an address has a single
//! spelling.

//! Every operation here is shown not to panic in `Base32Proofs.lean`.

#![allow(clippy::arithmetic_side_effects)]

/// Three bytes as five symbols, the last carrying one zero padding bit.
pub fn encode3(b: [u8; 3]) -> [u8; 5] {
    [
        b[0] / 8,
        (b[0] % 8) * 4 + b[1] / 64,
        b[1] / 2 % 32,
        (b[1] % 2) * 16 + b[2] / 16,
        (b[2] % 16) * 2,
    ]
}

/// The three bytes five symbols stand for, or nothing if a symbol is not
/// below 32 or the padding bit is set.
pub fn decode3(s: [u8; 5]) -> Option<[u8; 3]> {
    if s[0] >= 32 || s[1] >= 32 || s[2] >= 32 || s[3] >= 32 || s[4] >= 32 {
        return None;
    }
    if s[4] % 2 != 0 {
        return None;
    }
    Some([s[0] * 8 + s[1] / 4, (s[1] % 4) * 64 + s[2] * 2 + s[3] / 16, (s[3] % 16) * 16 + s[4] / 2])
}
