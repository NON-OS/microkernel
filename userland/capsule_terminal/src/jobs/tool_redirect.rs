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

//! A baked tool, `linux` among them, with its input or output redirected.
//! `< f` reads the file here and gives it to the program as its whole
//! input, then the end of input; `< /dev/null` is the end of input alone.
//! `> f` and `>> f` hold what it writes and put that in the file when it
//! ends. Everything that can be checked is checked before it is started,
//! and what cannot be honoured is refused then, in one line.

use super::classify::Verdict;
use super::stdin_queue::StdinQueue;
use super::tty;
use super::work::JobWork;
use crate::command::builtin::tool;
use crate::command::dispatch::{admit_tool, hears_end_of_input, is_operator, redirect_plan};
use crate::term::state::State;

#[path = "tool_redirect_ends.rs"]
mod tool_redirect_ends;
use tool_redirect_ends::ends;

/// `None` when `args` is not a tool with a redirect or a pipe, for the
/// ordinary dispatch to run. A line the redirect plan refuses is also left
/// to the dispatch, which says why.
pub(super) fn redirected(state: &mut State, args: &[&[u8]]) -> Option<Verdict> {
    if !args.iter().any(|a| is_operator(a)) {
        return None;
    }
    let plan = redirect_plan(args).ok()?;
    let name = *plan.words.first()?;
    if !tool::is_tool(name) {
        return None;
    }
    if let Err(line) = admit_tool(name, &plan) {
        return Some(refuse(state, &line));
    }
    let mut stdin = StdinQueue::new(hears_end_of_input(name));
    let (input, capture) = match ends(state, name, plan.input, plan.output) {
        Ok(ends) => ends,
        Err(line) => return Some(refuse(state, &line)),
    };
    let streams = tty::streams(capture.is_some(), input.is_some());
    let Some(pid) = tool::prepare(state, &plan.words, streams) else {
        return Some(Verdict::Handled);
    };
    if let Some(bytes) = input {
        /* Within `FILE_MAX`, checked as the file was read, so this holds. */
        let _ = stdin.whole(&bytes);
    }
    Some(Verdict::Job(JobWork::ExternalStage { pid, stdin, capture }))
}

fn refuse(state: &mut State, line: &[u8]) -> Verdict {
    state.scrollback.push_error(line);
    state.last_status = 1;
    Verdict::Handled
}
