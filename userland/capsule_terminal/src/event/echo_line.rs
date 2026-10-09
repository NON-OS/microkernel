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

use crate::term::context::context_line;
use crate::term::cwd::home_var;
use crate::term::dimensions::LINE_MAX;
use crate::term::identity::{hostname, username};
use crate::term::state::State;

/* Room for `user@host path % ` ahead of the longest command. */
const HEAD_MAX: usize = 256;

/*
 * Open the block with the line that ran: the prompt as it stood and `cmd`
 * after it, so the scrollback reads the way the input line did.
 */
pub(super) fn echo_line(state: &mut State, cmd: &[u8]) {
    let mut line = [0u8; LINE_MAX + HEAD_MAX];
    let (cwd, home) = (state.cwd.as_bytes(), home_var(state));
    let n = context_line(username(), hostname(), cwd, home, cmd, &mut line);
    state.scrollback.push_line(&line[..n]);
}
