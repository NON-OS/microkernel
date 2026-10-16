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

//! The line that says a background job has started.

use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

// "[n] started" line printed when a background job is submitted; the
// job's own output streams into the scrollback as Task 13's on_tick pump
// steps it.
pub(super) fn print_started(state: &mut State, id: u32) {
    let mut num = [0u8; 20];
    let nk = format_u64(id as u64, &mut num);
    let mut msg = [0u8; 32];
    let mut mk = 0;
    msg[mk] = b'[';
    mk += 1;
    mk += copy_into(&mut msg[mk..], &num[..nk]);
    mk += copy_into(&mut msg[mk..], b"] started");
    state.scrollback.push_line(&msg[..mk]);
}
