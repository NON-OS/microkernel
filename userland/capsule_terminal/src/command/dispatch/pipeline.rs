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

use alloc::vec::Vec;

use alloc::vec;

use super::exec::exec;
use super::filter::apply;
use super::filter::input::{not_a_filter, FILTERS};
use crate::term::state::State;

// Run a `a | b | c` pipeline: split the args on `|`, then fold each stage
// in order over an accumulating buffer. A stage whose command name is a
// known filter runs as a filter over the buffer; any other stage runs as
// a real command through `exec`, capturing its output as the new buffer.
// Only the filters read what is piped in. Any other command after a `|`
// used to run as if nothing had been piped and drop the lines without a
// word; it now stops the pipeline and says which commands read a pipe.
pub(super) fn run_pipeline(state: &mut State, args: &[&[u8]]) -> Vec<Vec<u8>> {
    let segments = split_stages(args);
    if segments.is_empty() {
        return Vec::new();
    }
    let mut lines = Vec::new();
    for (i, seg) in segments.iter().enumerate() {
        let stops = i > 0 && !is_filter(seg);
        lines = run_stage(state, seg, lines, i == 0);
        if stops {
            break;
        }
    }
    lines
}

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

pub(crate) fn split_stages<'a>(args: &'a [&'a [u8]]) -> Vec<&'a [&'a [u8]]> {
    let mut segments = Vec::new();
    let mut start = 0;
    for i in 0..=args.len() {
        if i == args.len() || args[i] == b"|" {
            if i > start {
                segments.push(&args[start..i]);
            }
            start = i + 1;
        }
    }
    segments
}

// Apply a `a | b | c` filter chain to pre-seeded input lines with no
// producer command: used when `< file` supplies the input instead of a
// leading command's captured output.
pub(super) fn run_filters(seed: Vec<Vec<u8>>, args: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut lines = seed;
    for seg in &split_stages(args) {
        lines = apply(seg, lines);
    }
    lines
}
