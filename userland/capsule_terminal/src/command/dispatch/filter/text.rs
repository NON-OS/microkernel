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

#[path = "text_cut.rs"]
mod text_cut;
#[path = "text_grep.rs"]
mod text_grep;
#[path = "text_reverse.rs"]
mod text_reverse;
#[path = "text_sort.rs"]
mod text_sort;
#[path = "text_uniq.rs"]
mod text_uniq;

pub(super) use text_cut::cut;
pub(super) use text_grep::grep;
pub(super) use text_reverse::{rev, tac};
pub(super) use text_sort::sort;
pub(super) use text_uniq::uniq;

/// The flags `name` was given, or the line saying why they were refused.
fn flags<'a>(name: &[u8], args: &[&'a [u8]]) -> Result<Parsed<'a>, Vec<Vec<u8>>> {
    match spec_for(name) {
        Some(spec) => parse(&spec, args).map_err(|e| vec![e]),
        None => Ok(Parsed::default()),
    }
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
