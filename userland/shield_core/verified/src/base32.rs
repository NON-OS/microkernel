//! The address alphabet: lower case RFC 4648 base32, no padding, one spelling per address.
//! The padding bit of the tail must be zero, or one address would have two spellings.
//! Symbols are cut by division and remainder, which a proof reads as digits.
//! `Base32Proofs.lean` proves every operation panic free, so the arithmetic lint is off.

#![allow(clippy::arithmetic_side_effects)]

/// The character for a symbol, or nothing for a value no symbol has.
pub fn glyph(value: u8) -> Option<u8> {
    if value < 26 {
        Some(b'a' + value)
    } else if value < 32 {
        Some(b'2' + (value - 26))
    } else {
        None
    }
}

/// The symbol a character stands for, or nothing outside the alphabet.
pub fn value(glyph: u8) -> Option<u8> {
    if glyph >= b'a' && glyph <= b'z' {
        Some(glyph - b'a')
    } else if glyph >= b'2' && glyph <= b'7' {
        Some(26 + (glyph - b'2'))
    } else {
        None
    }
}

/// Five bytes as eight symbols, each below 32.
pub fn encode5(b: [u8; 5]) -> [u8; 8] {
    [
        b[0] / 8,
        (b[0] % 8) * 4 + b[1] / 64,
        b[1] / 2 % 32,
        (b[1] % 2) * 16 + b[2] / 16,
        (b[2] % 16) * 2 + b[3] / 128,
        b[3] / 4 % 32,
        (b[3] % 4) * 8 + b[4] / 32,
        b[4] % 32,
    ]
}

/// The five bytes eight symbols stand for, or nothing if a symbol is not below 32.
pub fn decode5(s: [u8; 8]) -> Option<[u8; 5]> {
    if s[0] >= 32 || s[1] >= 32 || s[2] >= 32 || s[3] >= 32 {
        return None;
    }
    if s[4] >= 32 || s[5] >= 32 || s[6] >= 32 || s[7] >= 32 {
        return None;
    }
    Some([
        s[0] * 8 + s[1] / 4,
        (s[1] % 4) * 64 + s[2] * 2 + s[3] / 16,
        (s[3] % 16) * 16 + s[4] / 2,
        (s[4] % 2) * 128 + s[5] * 4 + s[6] / 8,
        (s[6] % 8) * 32 + s[7],
    ])
}
