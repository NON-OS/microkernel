// NONOS Operating System (AGPL-3.0-or-later)
//! The byte model for the two indirect fields (ACPI 6.5, 19.6.64 and
//! 19.6.7; recalled, unverified). An IndexField: the width as for a field,
//! but never bounded by a region; for each unit, the unit's byte offset is
//! written to the index register, then the data register is read or
//! written one unit wide; a Preserve write does that twice, reading first.
//! A BankField: the bank value is written to the select register once per
//! read or write, then the units go as for a field in that bank's bytes;
//! if a unit's bytes cover the select register itself, the units after it
//! go to whichever bank that write left selected, which is defined, not
//! refused.

use super::gen::Case;
use super::gen_ib::{select_at, DAT_AT, IDX_AT};
use super::model::{run, Log, Run};
use super::width::ones;
use std::collections::BTreeMap;

pub fn index_width(c: &Case) -> u64 {
    let (first, last) = (u64::from(c.off), u64::from(c.off + c.bits - 1));
    match c.access {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        _ => [1, 2, 4, 8].into_iter().find(|w| first / (8 * w) == last / (8 * w)).unwrap_or(1),
    }
}

pub fn index_run(c: &Case, hidden: &mut BTreeMap<u64, u8>, il: u64, write: Option<u128>) -> Run {
    let (w, mut out) = (index_width(c), Run { log: Log::new(), value: 0, fault: false });
    let (start, end) = (u64::from(c.off), u64::from(c.off + c.bits));
    for k in start / (8 * w)..=(end - 1) / (8 * w) {
        let (byte, lo, hi) = (k * w, start.max(k * 8 * w), end.min((k + 1) * 8 * w));
        let (shift, n) = (lo - k * 8 * w, hi - lo);
        let get = |h: &BTreeMap<u64, u8>| {
            (0..w).rev().fold(0u128, |v, i| v << 8 | u128::from(*h.get(&(byte + i)).unwrap_or(&0)))
        };
        let pick = (IDX_AT, il as u32, Some(byte));
        out.log.push(pick);
        let Some(v) = write else {
            out.log.push((DAT_AT, w as u32, None));
            out.value |= (get(hidden) >> shift & ones(n)) << (lo - start);
            continue;
        };
        let rest = match c.rule {
            _ if n == 8 * w => 0,
            0 => {
                out.log.extend([(DAT_AT, w as u32, None), pick]);
                get(hidden)
            }
            1 => ones(8 * w),
            _ => 0,
        };
        let new = (v >> (lo - start)) & ones(n);
        let word = (rest & !(ones(n) << shift) | new << shift) & ones(8 * w);
        out.log.push((DAT_AT, w as u32, Some(word as u64)));
        (0..w).for_each(|i| _ = hidden.insert(byte + i, (word >> (8 * i)) as u8));
    }
    out
}

pub fn bank_run(
    c: &Case,
    banks: &mut BTreeMap<(u8, u64), u8>,
    bank: u8,
    write: Option<u128>,
) -> Run {
    let mut mem: BTreeMap<u64, u8> =
        banks.iter().filter(|((b, _), _)| *b == bank).map(|((_, a), v)| (*a, *v)).collect();
    let mut out = run(c, &mut mem, write);
    out.log.insert(0, (select_at(c), 1, Some(u64::from(bank))));
    mem.into_iter().for_each(|(a, v)| _ = banks.insert((bank, a), v));
    out
}
