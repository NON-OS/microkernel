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

use super::manage;
use super::work::{parse, perform, report, USAGE};
use crate::command::output::Output;
use crate::term::state::State;

/// `pkg` on the window thread: `status`, a usage error, and an install or a
/// remove where it is not a job of its own (written to a file, piped on, or
/// with no worker thread to be had), which waits for the installer here as
/// it always did. A plain `pkg install` or `pkg remove` is a job (`job.rs`).
pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.first().copied() == Some(b"status") {
        return manage::status(state);
    }
    let Some(op) = parse(state.cwd.as_bytes(), args) else {
        state.scrollback.push_error(USAGE);
        return false;
    };
    let done = perform(&op);
    report(&mut Output::new(&mut state.scrollback), &done)
}
