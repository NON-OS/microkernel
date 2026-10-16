// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! `sort` over piped lines, by bytes or by leading integer.

use alloc::vec::Vec;

use super::{flags, uniq};

// sort [-n] [-r] [-u]: -n orders by leading integer, -r reverses, -u drops
// adjacent duplicates once the order is settled.
pub fn sort(args: &[&[u8]], mut input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let parsed = match flags(b"sort", args) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if parsed.has(b'n') {
        input.sort_unstable_by(|a, b| numeric_key(a).cmp(&numeric_key(b)).then_with(|| a.cmp(b)));
    } else {
        // Unstable sort: in place, no auxiliary allocation, and equal lines are
        // byte-identical so a stable order would not be observable anyway.
        input.sort_unstable();
    }
    if parsed.has(b'r') {
        input.reverse();
    }
    if parsed.has(b'u') {
        // `sort -u` collapses duplicates without counting them, so it asks
        // for the plain form regardless of what the pipeline stage was given.
        input = uniq(&[], input);
    }
    input
}

fn numeric_key(line: &[u8]) -> i64 {
    let body = line.trim_ascii_start();
    let (neg, digits) = match body.first() {
        Some(b'-') => (true, &body[1..]),
        _ => (false, body),
    };
    let mut v: i64 = 0;
    for &c in digits {
        if !c.is_ascii_digit() {
            break;
        }
        v = v.saturating_mul(10).saturating_add((c - b'0') as i64);
    }
    if neg {
        -v
    } else {
        v
    }
}
