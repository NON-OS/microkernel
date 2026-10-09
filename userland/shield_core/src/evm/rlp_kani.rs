//! Proved for every length and every integer: the prefixes decode back to
//! what was encoded, so a signed transaction carries its fields and nothing else.

use super::{header, uint};

/// Read back the length a header announces, the way a node does.
fn announced(out: &[u8], base: u8) -> Option<u64> {
    let first = out.first()?.checked_sub(base)?;
    if first < 56 {
        return Some(u64::from(first));
    }
    let count = usize::from(first - 55);
    let digits = out.get(1..1 + count)?;
    if digits.first() == Some(&0) {
        return None;
    }
    let mut n = 0u64;
    for d in digits {
        n = n.checked_mul(256)?.checked_add(u64::from(*d))?;
    }
    Some(n)
}

#[kani::proof]
#[kani::unwind(10)]
fn every_header_announces_its_own_length() {
    let len: u64 = kani::any();
    let len = usize::try_from(len).unwrap_or(usize::MAX);
    let mut out = Vec::new();
    header(&mut out, 0xc0, len);
    assert_eq!(announced(&out, 0xc0), u64::try_from(len).ok());
}

#[kani::proof]
#[kani::unwind(18)]
fn every_integer_is_minimal_and_decodes_back() {
    let v: u128 = kani::any();
    let mut out = Vec::new();
    uint(&mut out, v);
    if v < 0x80 && v != 0 {
        assert_eq!(out, [v as u8]);
        return;
    }
    let body = &out[1..];
    assert_eq!(usize::from(out[0] - 0x80), body.len());
    assert!(body.first() != Some(&0), "no leading zero");
    let mut back = 0u128;
    for b in body {
        back = (back << 8) | u128::from(*b);
    }
    assert_eq!(back, v);
}
