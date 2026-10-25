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

//! A redirected tool's two ends: the file read as its input, the capture of its output.

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::write_file;

use crate::command::dispatch::{not_written, Sink, Source};
use crate::jobs::capture::Capture;
use crate::term::cwd::resolve;
use crate::term::state::State;

#[path = "tool_redirect_input.rs"]
mod tool_redirect_input;
use tool_redirect_input::read_input;

/// Where the output goes: nowhere kept, or held for the file `> f` names,
/// which `>` empties first; the line saying why when it cannot be written.
fn capture_for(
    state: &State,
    name: &[u8],
    output: Sink<'_>,
) -> Result<Option<Box<Capture>>, Vec<u8>> {
    Ok(match output {
        Sink::Screen => None,
        Sink::Null => Some(Box::new(Capture::discard().split_by(name))),
        Sink::File { path, append } => {
            let path = resolve(state.cwd.as_bytes(), path);
            /* `>` empties the file before the program runs, as a shell
             * opens it, so a file that cannot be written is said now. */
            if !append {
                if let Err(e) = write_file(state.owner_pid, &path, b"") {
                    return Err(not_written(&path, e));
                }
            }
            Some(Box::new(Capture::to_file(state.owner_pid, path, append).split_by(name)))
        }
    })
}

/// The program's whole input, if it is given one, and the capture of its output.
type Ends = (Option<Vec<u8>>, Option<Box<Capture>>);

/// The input the program is given whole, if any, and where its output is
/// held, if anywhere; the line saying why when either end cannot be had.
pub(super) fn ends(
    state: &State,
    name: &[u8],
    input: Source<'_>,
    output: Sink<'_>,
) -> Result<Ends, Vec<u8>> {
    let input = match input {
        Source::Terminal => None,
        Source::Null => Some(Vec::new()),
        Source::File(path) => Some(read_input(state, name, path)?),
    };
    Ok((input, capture_for(state, name, output)?))
}
