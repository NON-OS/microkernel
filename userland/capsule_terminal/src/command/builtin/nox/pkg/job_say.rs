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

//! What a `pkg` request says when no worker takes it: still busy, or run on the window thread.

use alloc::vec::Vec;

use super::Prepared;
use crate::term::state::State;
use crate::term::util::format_u64;

pub(super) fn refuse(state: &mut State) -> Prepared {
    state
        .scrollback
        .push_error(b"pkg: the last request is still with the installer; try again shortly");
    state.last_status = 1;
    Prepared::Refused
}

/// No worker could be started (its stack not mapped, or the kernel refused
/// the thread). The request runs on the window thread instead, as it did
/// before workers, and the person is told the window will wait.
pub(super) fn say_inline(state: &mut State, errno: i64) {
    let mut num = [0u8; 24];
    let k = format_u64(errno.unsigned_abs(), &mut num);
    let mut line = said(b"pkg: no worker thread (-", &num[..k]);
    line.extend_from_slice(b"), the window waits for the installer");
    state.scrollback.push_line(&line);
}

pub(super) fn said(what: &[u8], tail: &[u8]) -> Vec<u8> {
    let mut line = Vec::with_capacity(what.len() + tail.len());
    line.extend_from_slice(what);
    line.extend_from_slice(tail);
    line
}
