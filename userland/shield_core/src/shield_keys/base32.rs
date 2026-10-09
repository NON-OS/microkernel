//! The address alphabet: lower case RFC 4648 base32 without padding, one canonical spelling.
//! The group arithmetic is the verified kernel's, proved in `lean-verified/Base32Proofs.lean`.

use nox_verified::base32::{encode5, glyph};
use nox_verified::base32_tail::encode3;

/// Bytes in a group, and symbols in a group.
pub(super) const GROUP: usize = 5;
pub(super) const SYMBOLS: usize = 8;
/// The bytes an address leaves after its groups, and their symbols.
pub(super) const TAIL: usize = 3;
pub(super) const TAIL_SYMBOLS: usize = 5;

/// Encode bytes whose length is a multiple of five, or three more. Nothing for other lengths.
pub fn encode(bytes: &[u8]) -> Option<String> {
    let (groups, tail) = bytes.as_chunks::<GROUP>();
    let mut out = String::with_capacity(bytes.len().saturating_mul(2));
    for group in groups {
        push(&mut out, &encode5(*group))?;
    }
    match <&[u8; TAIL]>::try_from(tail) {
        Ok(tail) => push(&mut out, &encode3(*tail))?,
        Err(_) if tail.is_empty() => {}
        Err(_) => return None,
    }
    Some(out)
}

/// Symbols as characters. Symbols are below 32, and the question mark keeps that checked.
fn push(out: &mut String, symbols: &[u8]) -> Option<()> {
    for symbol in symbols {
        out.push(char::from(glyph(*symbol)?));
    }
    Some(())
}
