//! Reading an address back. Any foreign character, bad group count or set padding bit is refused.
//! The decoder accepts only the encoder's output, proved in `lean-verified/Base32Proofs.lean`.

use super::base32::{GROUP, SYMBOLS, TAIL, TAIL_SYMBOLS};
use nox_verified::base32::{decode5, value};
use nox_verified::base32_tail::decode3;

/// Decode an encoding this module produced.
pub fn decode(text: &str) -> Option<Vec<u8>> {
    let mut symbols = Vec::with_capacity(text.len());
    for c in text.as_bytes() {
        symbols.push(value(*c)?);
    }
    let (groups, tail) = symbols.as_chunks::<SYMBOLS>();
    let mut out = Vec::with_capacity(groups.len().saturating_mul(GROUP).saturating_add(TAIL));
    for group in groups {
        out.extend_from_slice(&decode5(*group)?);
    }
    match <&[u8; TAIL_SYMBOLS]>::try_from(tail) {
        Ok(tail) => out.extend_from_slice(&decode3(*tail)?),
        Err(_) if tail.is_empty() => {}
        Err(_) => return None,
    }
    Some(out)
}
