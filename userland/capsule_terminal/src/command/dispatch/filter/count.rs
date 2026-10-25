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

#[path = "count_wc.rs"]
mod count_wc;

pub(super) use count_wc::wc;
pub use count_wc::{wc_row, word_count};

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
