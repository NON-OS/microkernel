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

//! `grep` over piped lines: keep the lines that hold a pattern.

use alloc::vec;
use alloc::vec::Vec;

use super::flags;
use crate::term::util::format_u64;

// grep [-c -i -n -v] <pattern>: keep matching lines; -i ignores case, -v
// inverts, -n numbers each kept line by its place in the input, -c prints only
// how many matched. It used to know only -i and -v and take any other flag
// for the pattern.
pub(in super::super) fn grep(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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
