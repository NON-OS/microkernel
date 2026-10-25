// NONOS Operating System (AGPL-3.0-or-later)
//! The byte model, stated apart from the interpreter. The width: ByteAcc 1,
//! WordAcc 2, DWordAcc 4, QWordAcc 8 bytes; AnyAcc and BufferAcc the
//! narrowest of those whose one aligned unit holds the whole unit and ends
//! inside the region, else 1;
//! an embedded controller always 1 (ACPI 6.5, 19.6.48 and 12.3; recalled,
//! unverified; the AnyAcc choice is ours). Units are aligned to the
//! region's start and taken in rising order; one past the region's end is
//! a fault, after the units before it. A read takes each unit once. A
//! write puts a whole unit as it is; a part by the rule: Preserve reads the
//! unit first, WriteAsOnes fills with ones, WriteAsZeros with zeros.

use super::gen::Case;
use super::width::{ones, width};
use std::collections::BTreeMap;

pub type Log = Vec<(u64, u32, Option<u64>)>;

pub struct Run {
    pub log: Log,
    pub value: u128,
    pub fault: bool,
}

/// Reads when `write` is None, else writes; `mem` is the bytes by address.
pub fn run(c: &Case, mem: &mut BTreeMap<u64, u8>, write: Option<u128>) -> Run {
    let (w, mut out) = (width(c), Run { log: Vec::new(), value: 0, fault: false });
    let (start, end) = (u64::from(c.off), u64::from(c.off + c.bits));
    for k in start / (8 * w)..=(end - 1) / (8 * w) {
        let (byte, lo, hi) = (k * w, start.max(k * 8 * w), end.min((k + 1) * 8 * w));
        if byte + w > c.len {
            out.fault = true;
            return out;
        }
        let at = c.base + byte;
        let get = |mem: &BTreeMap<u64, u8>| {
            (0..w).rev().fold(0u128, |v, i| v << 8 | u128::from(mem[&(at + i)]))
        };
        let (shift, n) = (lo - k * 8 * w, hi - lo);
        let Some(v) = write else {
            out.log.push((at, w as u32, None));
            out.value |= (get(mem) >> shift & ones(n)) << (lo - start);
            continue;
        };
        let new = (v >> (lo - start)) & ones(n);
        let rest = match c.rule {
            _ if n == 8 * w => 0,
            0 => {
                out.log.push((at, w as u32, None));
                get(mem)
            }
            1 => ones(8 * w),
            _ => 0,
        };
        let word = (rest & !(ones(n) << shift) | new << shift) & ones(8 * w);
        out.log.push((at, w as u32, Some(word as u64)));
        (0..w).for_each(|i| *mem.get_mut(&(at + i)).unwrap() = (word >> (8 * i)) as u8);
    }
    out
}
