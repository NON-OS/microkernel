//! Recursive length prefix, the encoding a signed transaction travels in.
//! Integers are big endian without leading zeros, and zero is the empty string.

/// A byte string. A single byte below 0x80 is its own encoding.
pub(super) fn bytes(out: &mut Vec<u8>, b: &[u8]) {
    if let [one] = b {
        if *one < 0x80 {
            out.push(*one);
            return;
        }
    }
    header(out, 0x80, b.len());
    out.extend_from_slice(b);
}

/// An unsigned integer.
pub(super) fn uint(out: &mut Vec<u8>, v: u128) {
    let be = v.to_be_bytes();
    bytes(out, trimmed(&be));
}

/// A 32-byte word as an integer, for the two halves of a signature.
pub(super) fn word(out: &mut Vec<u8>, w: &[u8; 32]) {
    bytes(out, trimmed(w));
}

/// A list whose items are already encoded, in order, in `items`.
pub(super) fn list(out: &mut Vec<u8>, items: &[u8]) {
    header(out, 0xc0, items.len());
    out.extend_from_slice(items);
}

fn trimmed(be: &[u8]) -> &[u8] {
    let skip = be.iter().take_while(|b| **b == 0).count();
    be.get(skip..).unwrap_or_default()
}

/// The prefix for a payload of `len` bytes, in the long form past 55 bytes.
pub(super) fn header(out: &mut Vec<u8>, base: u8, len: usize) {
    match u8::try_from(len) {
        Ok(short) if short < 56 => out.push(base.saturating_add(short)),
        _ => {
            let be = u64::try_from(len).unwrap_or(u64::MAX).to_be_bytes();
            let digits = trimmed(&be);
            let count = u8::try_from(digits.len()).unwrap_or(8);
            out.push(base.saturating_add(55).saturating_add(count));
            out.extend_from_slice(digits);
        }
    }
}

#[cfg(kani)]
#[path = "rlp_kani.rs"]
mod rlp_kani;
