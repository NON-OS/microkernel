// NONOS Operating System (AGPL-3.0-or-later)
//! Random integer expressions and their values, from the operators'
//! definitions: the model the generated programs are checked against.

use crate::enc::*;

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

const BIN: [u16; 16] = [
    0x72, 0x74, 0x77, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F, 0x79, 0x7A, 0x85, 0x90, 0x91, 0x93, 0x94, 0x95,
];

/// An expression and its value under the model, or None when it divides by
/// zero.
pub fn gen(s: &mut u64, depth: u32, mask: u64) -> (Vec<u8>, Option<u64>) {
    let pick = next(s);
    if depth == 0 || pick.is_multiple_of(5) {
        let v = match next(s) % 4 {
            0 => next(s) % 4,
            1 => next(s) & 0xFF,
            _ => next(s),
        };
        return (int(v), Some(v & mask));
    }
    if pick % 7 == 1 {
        let (a, x) = gen(s, depth - 1, mask);
        return match next(s) % 3 {
            0 => (e(0x80, &[&a, &[0]]), x.map(|x| !x & mask)),
            1 => (e(0x92, &[&a]), x.map(|x| if x == 0 { mask } else { 0 })),
            _ => (e(0x81, &[&a, &[0]]), x.map(|x| 64 - x.leading_zeros() as u64)),
        };
    }
    let code = BIN[(next(s) % BIN.len() as u64) as usize];
    let (a, x) = gen(s, depth - 1, mask);
    let (b, y) = gen(s, depth - 1, mask);
    let t = |c: bool| if c { mask } else { 0 };
    let v = match (x, y) {
        (Some(x), Some(y)) => match code {
            0x72 => Some(x.wrapping_add(y)),
            0x74 => Some(x.wrapping_sub(y)),
            0x77 => Some(x.wrapping_mul(y)),
            0x7B => Some(x & y),
            0x7C => Some(!(x & y)),
            0x7D => Some(x | y),
            0x7E => Some(!(x | y)),
            0x7F => Some(x ^ y),
            0x79 => Some(if y >= 64 { 0 } else { x << y }),
            0x7A => Some(if y >= 64 { 0 } else { x >> y }),
            0x85 => x.checked_rem(y),
            0x90 => Some(t(x != 0 && y != 0)),
            0x91 => Some(t(x != 0 || y != 0)),
            0x93 => Some(t(x == y)),
            0x94 => Some(t(x > y)),
            _ => Some(t(x < y)),
        },
        _ => None,
    };
    let enc = if code >= 0x90 { e(code, &[&a, &b]) } else { e(code, &[&a, &b, &[0]]) };
    (enc, v.map(|v| v & mask))
}
