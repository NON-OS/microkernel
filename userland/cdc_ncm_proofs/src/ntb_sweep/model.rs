// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound reader takes from a received NTB (USB NCM 1.0 sections 3.2
//! and 3.3, as recalled, unverified), restated apart from the driver: the
//! header of its format (NTB32 only when the device offered it), the block
//! no longer than the transfer or the input size; then each NDP along the
//! next-index chain, read once, that lies wholly in the block with room for
//! one entry and its terminator; of an NDP signed for datagrams without a
//! CRC, its entries in order to the first that is zero, shorter than an
//! Ethernet header or not wholly in the block; forty at most in all.

use super::shape::{N16, N32};

const MOST: usize = 40;

fn le(b: &[u8], at: usize, width: usize) -> Option<usize> {
    let w = b.get(at..at.checked_add(width)?)?;
    Some(w.iter().rev().fold(0usize, |v, &x| v << 8 | x as usize))
}

pub fn datagrams(raw: &[u8], rx_max: usize, offered32: bool) -> Vec<(usize, usize)> {
    let s = match le(raw, 0, 4) {
        Some(x) if x == N16.nth_sign as usize => &N16,
        Some(x) if offered32 && x == N32.nth_sign as usize => &N32,
        _ => return Vec::new(),
    };
    let w = if s.wide { 4 } else { 2 };
    let (Some(hlen), Some(blen)) = (le(raw, 4, 2), le(raw, 8, w)) else { return Vec::new() };
    let body = if blen == 0 { Some(raw) } else { raw.get(..blen) };
    let first = le(raw, if s.wide { 12 } else { 10 }, w);
    let (Some(b), Some(mut at)) = (body, first) else { return Vec::new() };
    if hlen != s.nth_len || b.len() > rx_max || at < s.nth_len {
        return Vec::new();
    }
    let (mut out, mut seen) = (Vec::new(), Vec::new());
    while at != 0 && at >= s.nth_len && !seen.contains(&at) {
        seen.push(at);
        let Some(len) = le(b, at + 4, 2) else { break };
        if len < s.ndp_header + 2 * s.entry || at + len > b.len() {
            break;
        }
        let next = le(b, at + s.ndp_next, w).unwrap();
        if le(b, at, 4) == Some(s.ndp_sign as usize) {
            for k in 0..(len - s.ndp_header) / s.entry - 1 {
                let e = at + s.ndp_header + k * s.entry;
                let (i, n) = (le(b, e, w).unwrap(), le(b, e + w, w).unwrap());
                if out.len() >= MOST || i == 0 || n < 14 || i + n > b.len() {
                    break;
                }
                out.push((i, n));
            }
        }
        at = next;
    }
    out
}
