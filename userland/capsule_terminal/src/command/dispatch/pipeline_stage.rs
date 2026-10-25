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

//! One pipe stage: a filter over the lines before it, or the command that starts the pipe.

use alloc::vec;
use alloc::vec::Vec;

use super::super::exec::exec;
use super::super::filter::apply;
use super::super::filter::input::{not_a_filter, FILTERS};
use crate::term::state::State;

/// Whether a stage reads the lines piped into it.
pub(crate) fn is_filter(seg: &[&[u8]]) -> bool {
    FILTERS.contains(&seg.first().copied().unwrap_or(b""))
}

/// One stage over the lines before it. `first` is the stage nothing is piped
/// into, the only place a command that is not a filter can stand.
pub(crate) fn run_stage(
    state: &mut State,
    seg: &[&[u8]],
    buffer: Vec<Vec<u8>>,
    first: bool,
) -> Vec<Vec<u8>> {
    if is_filter(seg) {
        return apply(seg, buffer);
    }
    if !first {
        state.last_status = 1;
        return vec![not_a_filter(seg.first().copied().unwrap_or(b""))];
    }
    state.scrollback.begin_capture();
    let _ = exec(state, seg);
    state.scrollback.end_capture()
}
