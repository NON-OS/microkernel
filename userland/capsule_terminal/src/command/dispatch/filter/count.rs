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

//! `wc`, `head` and `tail` over a pipe's lines, with the flags `help` gives
//! them. In a pipe they used to ignore every flag: `ls | head -n 3` printed
//! ten lines and `ls | wc -w` counted lines.

use alloc::vec;
use alloc::vec::Vec;

use super::input::spec_for;
use crate::command::flags::{parse, parse_usize};
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

pub(super) fn wc(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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

/// The count `head` and `tail` take in a pipe: `-n 5`, `-n5`, `-5` or a bare
/// `5`, ten by default.
pub fn line_count(name: &[u8], args: &[&[u8]]) -> Result<usize, Vec<u8>> {
    let Some(spec) = spec_for(name) else { return Ok(10) };
    let parsed = parse(&spec, args)?;
    if let Some(extra) = parsed.operands.first() {
        if let (Some(n), 1) = (parse_usize(extra), parsed.operands.len()) {
            return Ok(n);
        }
        let mut msg = Vec::from(name);
        msg.extend_from_slice(b": in a pipe there is no file to read, not ");
        msg.extend_from_slice(extra);
        return Err(msg);
    }
    match parsed.value(b'n') {
        None => Ok(10),
        Some(v) => parse_usize(v).ok_or_else(|| {
            let mut msg = Vec::from(name);
            msg.extend_from_slice(b": -n takes a count");
            msg
        }),
    }
}

pub(super) fn head(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    match line_count(b"head", args) {
        Ok(n) => input.into_iter().take(n).collect(),
        Err(e) => vec![e],
    }
}

pub(super) fn tail(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    match line_count(b"tail", args) {
        Ok(n) => {
            let skip = input.len().saturating_sub(n);
            input.into_iter().skip(skip).collect()
        }
        Err(e) => vec![e],
    }
}
