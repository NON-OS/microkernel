// NONOS Operating System (AGPL-3.0-or-later)
//! The receive framing's model, written here from Linux v6.12's parser
//! (read for behaviour), and random transfers, most with a trailer that
//! points inside them so their table is walked.

use crate::ax::link::{RX_LEN, SPANS_MAX};

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

/// The model: whole frames between the transfer's start and its table.
pub fn model(t: &[u8]) -> Vec<(usize, usize)> {
    let word = |at: usize| u32::from_le_bytes(t[at..at + 4].try_into().unwrap());
    let Some(end) = t.len().checked_sub(4) else { return vec![] };
    let (count, table) = ((word(end) & 0xffff) as usize, (word(end) >> 16) as usize);
    if table + 4 * count > end {
        return vec![];
    }
    let (mut cursor, mut out) = (0, vec![]);
    for e in (0..count).map(|i| word(table + 4 * i)) {
        let len = (e >> 16 & 0x1fff) as usize;
        if len == 0 {
            continue;
        }
        if cursor + len > table {
            break;
        }
        let good = e & (1 << 29 | 1 << 31) == 0 && (16..=1520).contains(&len);
        if good && out.len() < SPANS_MAX {
            out.push((cursor + 2, (len - 2).min(1514)));
        }
        cursor += len.div_ceil(8) * 8;
    }
    out
}

/// A table entry: half the time a length a frame could have, with marks.
fn entry(seed: &mut u64) -> u32 {
    let len = (next(seed) % 1600) as u32;
    let marks = next(seed) as u32 & 0xA000_0000;
    if next(seed).is_multiple_of(2) {
        len << 16 | marks
    } else {
        next(seed) as u32
    }
}

pub fn transfer(seed: &mut u64) -> Vec<u8> {
    let len = (next(seed) % (RX_LEN as u64)) as usize;
    let mut t: Vec<u8> = (0..len).map(|_| next(seed) as u8).collect();
    if len >= 8 && !next(seed).is_multiple_of(4) {
        let table = (next(seed) % (len as u64 - 3)) as usize & !3;
        let count = (next(seed) % ((len - 4 - table) as u64 / 4 + 3)) as u32;
        for at in (table..len - 4).step_by(4) {
            let (e, room) = (entry(seed), 4.min(len - 4 - at));
            t[at..at + room].copy_from_slice(&e.to_le_bytes()[..room]);
        }
        t[len - 4..].copy_from_slice(&(count | (table as u32) << 16).to_le_bytes());
    }
    t
}
