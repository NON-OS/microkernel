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

//! `wc` over piped lines, and the row the file `wc` prints too.

use alloc::vec;
use alloc::vec::Vec;

use super::super::input::spec_for;
use crate::command::flags::parse;
use crate::term::util::format_u64;

/// One `wc` row: the counts asked for (all three when none is), labelled, in
/// the order lines, words, bytes. The file `wc` prints the same row.
pub fn wc_row(lines: u64, words: u64, bytes: u64, want: [bool; 3]) -> Vec<u8> {
    let picked = want.iter().any(|w| *w);
    let mut row = Vec::new();
    for (i, (label, v)) in
        [(&b"lines "[..], lines), (b"words ", words), (b"bytes ", bytes)].into_iter().enumerate()
    {
        if picked && !want[i] {
            continue;
        }
        if !row.is_empty() {
            row.extend_from_slice(b"  ");
        }
        row.extend_from_slice(label);
        let mut num = [0u8; 20];
        let n = format_u64(v, &mut num);
        row.extend_from_slice(&num[..n]);
    }
    row
}

/// Words in `bytes`: runs of anything but ASCII whitespace.
pub fn word_count(bytes: &[u8]) -> u64 {
    bytes.split(|b| b.is_ascii_whitespace()).filter(|w| !w.is_empty()).count() as u64
}

pub(in super::super) fn wc(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let Some(spec) = spec_for(b"wc") else { return Vec::new() };
    let parsed = match parse(&spec, args) {
        Ok(p) => p,
        Err(e) => return vec![e],
    };
    let words = input.iter().map(|l| word_count(l)).sum();
    // Each line arrived ending in a newline, which is a byte as in a file.
    let bytes = input.iter().map(|l| l.len() as u64 + 1).sum();
    let want = [parsed.has(b'l'), parsed.has(b'w'), parsed.has(b'c')];
    vec![wc_row(input.len() as u64, words, bytes, want)]
}
