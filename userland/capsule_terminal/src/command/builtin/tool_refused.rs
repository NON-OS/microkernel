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

//! What the terminal says when the kernel will not start a tool.

use crate::term::state::State;

use super::tool_busy::{running_said, ERRNO_EXIST};

const ERRNO_NOENT: i64 = -2;

/// Which tool the kernel would not start, and why, in its own words.
pub fn refused(state: &mut State, name: &[u8], rc: i64) {
    let why: &[u8] = match rc {
        ERRNO_NOENT => b": not installed in this build",
        /* The kernel's answer when a live one holds the tool's endpoints. */
        ERRNO_EXIST => running_said(name),
        _ => b": the kernel refused to start it",
    };
    let mut line = name.to_vec();
    line.extend_from_slice(why);
    state.scrollback.push_error(&line);
    state.last_status = 1;
}
