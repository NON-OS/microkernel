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

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{read_file, write_file};

use super::capture::Capture;
use super::classify::Verdict;
use super::stdin_queue::StdinQueue;
use super::tty;
use super::work::JobWork;
use crate::command::builtin::tool;
use crate::command::dispatch::{
    admit_tool, hears_end_of_input, is_operator, not_written, redirect_plan, Sink, Source,
};
use crate::term::cwd::resolve;
use crate::term::state::State;

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
    let input = match plan.input {
        Source::Terminal => None,
        Source::Null => Some(Vec::new()),
        Source::File(path) => match read_input(state, name, path) {
            Ok(bytes) => Some(bytes),
            Err(line) => return Some(refuse(state, &line)),
        },
    };
    let capture = match plan.output {
        Sink::Screen => None,
        Sink::Null => Some(Box::new(Capture::discard().split_by(name))),
        Sink::File { path, append } => {
            let path = resolve(state.cwd.as_bytes(), path);
            /* `>` empties the file before the program runs, as a shell
             * opens it, so a file that cannot be written is said now. */
            if !append {
                if let Err(e) = write_file(state.owner_pid, &path, b"") {
                    return Some(refuse(state, &not_written(&path, e)));
                }
            }
            Some(Box::new(Capture::to_file(state.owner_pid, path, append).split_by(name)))
        }
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

/// The file `< path` names, whole, or the line saying why not.
fn read_input(state: &State, name: &[u8], path: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
    let full = resolve(state.cwd.as_bytes(), path);
    let at = |why: &[u8]| [name, b": ", &full[..], b": ", why].concat();
    /* One byte past the most, so a file too big is seen, never cut short. */
    match read_file(state.owner_pid, &full, StdinQueue::FILE_MAX as u32 + 1) {
        Ok(bytes) if bytes.len() > StdinQueue::FILE_MAX => {
            Err(at(b"larger than a program's input takes (1 MiB)"))
        }
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(at(e.as_bytes())),
    }
}

fn refuse(state: &mut State, line: &[u8]) -> Verdict {
    state.scrollback.push_error(line);
    state.last_status = 1;
    Verdict::Handled
}
