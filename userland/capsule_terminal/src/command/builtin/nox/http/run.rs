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

use nonos_libc::mk_yield;

use super::job::prepare;
use crate::command::output::Output;
use crate::jobs::JobProgress;
use crate::term::state::State;

/// `http` where it is not a job of its own: written to a file or piped on
/// (`http host > page`), it runs inline as it always did, through the same
/// stepped exchange, and the window waits for it.
pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(mut job) = prepare(state, args) else {
        return false;
    };
    let mut out = Output::new(&mut state.scrollback);
    loop {
        if let JobProgress::Done(status) = job.step_once(&mut out) {
            return status == 0;
        }
        mk_yield();
    }
}
