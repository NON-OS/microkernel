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

use alloc::vec;
use alloc::vec::Vec;

use super::input::spec_for;
use crate::command::flags::{parse, Parsed};
use crate::term::util::format_u64;

/// The flags `name` was given, or the line saying why they were refused.
fn flags<'a>(name: &[u8], args: &[&'a [u8]]) -> Result<Parsed<'a>, Vec<Vec<u8>>> {
    match spec_for(name) {
        Some(spec) => parse(&spec, args).map_err(|e| vec![e]),
        None => Ok(Parsed::default()),
    }
}

// grep [-c -i -n -v] <pattern>: keep matching lines; -i ignores case, -v
// inverts, -n numbers each kept line by its place in the input, -c prints only
// how many matched. It used to know only -i and -v and take any other flag
// for the pattern.
pub(super) fn grep(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let parsed = match flags(b"grep", args) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let Some(&pat) = parsed.operands.first() else {
        return vec![Vec::from(&b"grep: missing pattern"[..])];
    };
    let (ci, inv) = (parsed.has(b'i'), parsed.has(b'v'));
    let kept = input.into_iter().enumerate().filter(|(_, l)| contains(l, pat, ci) != inv);
    if parsed.has(b'c') {
        let mut num = [0u8; 24];
        let n = format_u64(kept.count() as u64, &mut num);
        return vec![num[..n].to_vec()];
    }
    if !parsed.has(b'n') {
        return kept.map(|(_, l)| l).collect();
    }
    kept.map(|(i, l)| {
        let mut num = [0u8; 24];
        let n = format_u64(i as u64 + 1, &mut num);
        let mut row = num[..n].to_vec();
        row.push(b':');
        row.extend_from_slice(&l);
        row
    })
    .collect()
}

// sort [-n] [-r] [-u]: -n orders by leading integer, -r reverses, -u drops
// adjacent duplicates once the order is settled.
pub(super) fn sort(args: &[&[u8]], mut input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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

/// Collapse runs of equal lines. `-c` prefixes each with how many there were,
/// which is the form this is nearly always reached for: `sort | uniq -c` is
/// how anyone counts anything from a listing.
pub(super) fn uniq(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let count = match flags(b"uniq", args) {
        Ok(p) => p.has(b'c'),
        Err(e) => return e,
    };
    let mut out: Vec<Vec<u8>> = Vec::new();
    let mut runs: Vec<u64> = Vec::new();
    for line in input {
        if out.last().map(|prev| prev == &line).unwrap_or(false) {
            if let Some(n) = runs.last_mut() {
                *n += 1;
            }
            continue;
        }
        out.push(line);
        runs.push(1);
    }
    if !count {
        return out;
    }
    out.into_iter()
        .zip(runs)
        .map(|(line, n)| {
            // Right aligned in a fixed column so the counts form a column of
            // their own and the lines beside them still line up.
            let mut num = [0u8; 24];
            let k = format_u64(n, &mut num);
            let mut row = Vec::with_capacity(6 + 1 + line.len());
            row.resize(6usize.saturating_sub(k), b' ');
            row.extend_from_slice(&num[..k]);
            row.push(b' ');
            row.extend_from_slice(&line);
            row
        })
        .collect()
}

/// Reverse the order of the lines. The counterpart to `tail` when what is
/// wanted is the whole thing, newest first.
pub(super) fn tac(mut input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    input.reverse();
    input
}

/// Reverse the bytes within each line, leaving the order of lines alone.
pub(super) fn rev(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    input
        .into_iter()
        .map(|mut line| {
            line.reverse();
            line
        })
        .collect()
}

pub(super) fn nl(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len());
    for (i, line) in input.iter().enumerate() {
        let mut num = [0u8; 24];
        let k = format_u64(i as u64 + 1, &mut num);
        let mut row = Vec::with_capacity(k + 2 + line.len());
        row.extend_from_slice(&num[..k]);
        row.extend_from_slice(b"  ");
        row.extend_from_slice(line);
        out.push(row);
    }
    out
}

// cut -d <delim> -f <n>: emit the n-th delim-separated field of each line
// (1-based; default delimiter space, default field 1). Missing field -> "".
// The value may follow its flag or be joined to it, -d, or -d ",".
pub(super) fn cut(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let parsed = match flags(b"cut", args) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let delim = parsed.value(b'd').and_then(|d| d.first().copied()).unwrap_or(b' ');
    let field = parsed.value(b'f').map(field_index).unwrap_or(1);
    input
        .into_iter()
        .map(|l| l.split(|&b| b == delim).nth(field - 1).map(<[u8]>::to_vec).unwrap_or_default())
        .collect()
}

fn field_index(digits: &[u8]) -> usize {
    let mut n = 0usize;
    for &c in digits {
        if c.is_ascii_digit() {
            n = n.saturating_mul(10).saturating_add((c - b'0') as usize);
        }
    }
    n.max(1)
}

fn contains(hay: &[u8], needle: &[u8], ci: bool) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > hay.len() {
        return false;
    }
    (0..=hay.len() - needle.len()).any(|i| {
        needle.iter().enumerate().all(|(j, &nb)| {
            if ci {
                hay[i + j].eq_ignore_ascii_case(&nb)
            } else {
                hay[i + j] == nb
            }
        })
    })
}
