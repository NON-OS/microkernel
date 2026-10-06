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

//! `sort f`, `uniq f`, `cut -d , -f 2 f`, `nl f`, `tac f`, `rev f`: a line
//! filter over the files named after it. `help` lists these as text commands,
//! but they ran only after a `|`; named on their own they answered "not
//! found". The filter is the pipe's own, over the files' lines in order.

use alloc::vec::Vec;

use super::read_file::slurp;
use crate::command::dispatch::{apply_filter, lines_of, spec_for, without};
use crate::command::flags::parse;
use crate::command::output::Output;
use crate::term::state::State;

pub fn filter_file(state: &mut State, argv: &[&[u8]]) {
    let name = argv[0];
    let Some(spec) = spec_for(name) else { return };
    let parsed = match parse(&spec, &argv[1..]) {
        Ok(p) => p,
        Err(e) => {
            Output::new(&mut state.scrollback).writeln(&e);
            state.last_status = 1;
            return;
        }
    };
    if parsed.operands.is_empty() {
        let mut msg = Vec::from(name);
        msg.extend_from_slice(b": give a file, or pipe into it: cat f | ");
        msg.extend_from_slice(name);
        Output::new(&mut state.scrollback).writeln(&msg);
        state.last_status = 1;
        return;
    }
    let mut lines = Vec::new();
    for file in &parsed.operands {
        let Some(bytes) = slurp(state, file) else {
            state.last_status = 1;
            return;
        };
        lines.extend(lines_of(&bytes));
    }
    let seg = without(argv, &parsed.operands);
    let mut out = Output::new(&mut state.scrollback);
    for line in apply_filter(&seg, lines) {
        out.writeln(&line);
    }
    state.last_status = 0;
}
