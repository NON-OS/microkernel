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

//! One turn of whichever fetch the directory bootstrap wants.

extern crate alloc;

use alloc::vec::Vec;

use super::super::state::Manager;
use super::poll::Poll;
use super::start::start;

/// Where a fetch stands after this turn.
pub(in crate::manager) enum Turn {
    /// Still going; ask again next turn with the same target.
    Busy,
    /// The inflated body.
    Got(Vec<u8>),
    /// This target gave nothing; move on to the next.
    Missed,
}

/// Start the fetch of `path` from an authority, or advance the one in
/// flight. The caller names the same target every turn until it is no
/// longer Busy, so the fetch in flight is always the one it asked for.
pub(in crate::manager) fn turn(
    state: &mut Manager,
    address: [u8; 4],
    dir_port: u16,
    path: &[u8],
) -> Turn {
    let tcp_port = state.tcp_port;
    let Some(job) = state.dir.job.as_mut() else {
        return match start(tcp_port, address, dir_port, path) {
            Some(job) => {
                state.dir.job = Some(job);
                Turn::Busy
            }
            None => Turn::Missed,
        };
    };
    let result = match job.poll(tcp_port) {
        Poll::Pending => return Turn::Busy,
        Poll::Done(body) => Turn::Got(body),
        Poll::Failed => Turn::Missed,
    };
    state.dir.job = None;
    result
}
