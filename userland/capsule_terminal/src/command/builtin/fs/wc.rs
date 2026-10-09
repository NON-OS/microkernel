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

//! Count lines, words and bytes in files; -l, -w and -c select columns. With
//! more than one file each row is named and a total follows; it used to count
//! the first file and drop the rest.

use alloc::vec::Vec;

use super::read_file::slurp;
use crate::command::dispatch::{spec_for, wc_row, word_count};
use crate::command::flags::parse;
use crate::command::output::Output;
use crate::term::state::State;

pub fn wc(state: &mut State, argv: &[&[u8]]) {
    let Some(spec) = spec_for(b"wc") else { return };
    let parsed = match parse(&spec, &argv[1..]) {
        Ok(p) => p,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    if parsed.operands.is_empty() {
        Output::new(&mut state.scrollback).writeln(b"wc: missing file");
        return;
    }
    let want = [parsed.has(b'l'), parsed.has(b'w'), parsed.has(b'c')];
    let many = parsed.operands.len() > 1;
    let mut total = [0u64; 3];
    for file in &parsed.operands {
        let Some(bytes) = slurp(state, file) else { continue };
        let counts = [
            bytes.iter().filter(|&&b| b == b'\n').count() as u64,
            word_count(&bytes),
            bytes.len() as u64,
        ];
        for (t, c) in total.iter_mut().zip(counts) {
            *t += c;
        }
        let mut row = wc_row(counts[0], counts[1], counts[2], want);
        if many {
            row.extend_from_slice(b"  ");
            row.extend_from_slice(file);
        }
        Output::new(&mut state.scrollback).writeln(&row);
    }
    if many {
        let mut row: Vec<u8> = wc_row(total[0], total[1], total[2], want);
        row.extend_from_slice(b"  total");
        Output::new(&mut state.scrollback).writeln(&row);
    }
}
