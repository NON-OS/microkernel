//! Addresses written into the build as text and read at compile time.
//!
//! The indexing and arithmetic below run in the compiler, on constant input,
//! and a bad digit fails the build. Nothing here runs on a phone.
#![allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]

/// Forty hex digits as twenty bytes, at compile time, so a typo in an address
/// is a build failure, not a wrong recipient.
pub(crate) const fn hex20(text: &str) -> [u8; 20] {
    let src = text.as_bytes();
    assert!(src.len() == 40, "an address is forty hex digits");
    let mut out = [0u8; 20];
    let mut i = 0;
    while i < 20 {
        out[i] = (nibble(src[2 * i]) << 4) | nibble(src[2 * i + 1]);
        i += 1;
    }
    out
}

/// A lowercase hex digit's value. Any other byte indexes one past the end of
/// a one-element table, which stops the compiler on this line: a const fn
/// cannot return an error, and a sentinel would decode to a plausible byte.
const fn nibble(c: u8) -> u8 {
    let (value, bad) = match c {
        b'0'..=b'9' => (c - b'0', 0),
        b'a'..=b'f' => (c - b'a' + 10, 0),
        _ => (0, 1),
    };
    let refuse = [0u8; 1];
    value | refuse[bad]
}
