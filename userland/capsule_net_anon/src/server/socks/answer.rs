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

//! Where a SOCKS frame leaves the serve loop.

use nonos_libc::mk_ipc_reply;

use crate::manager::Manager;

use super::anyone::Anyone;
use super::frame::is_socks;
use super::front::Front;

/// Answer `frame` from `pid` if it is a SOCKS frame. `false` leaves it to
/// the API, whose requests open with the magic rather than 0, 1 or 2.
pub fn answer(front: &mut Front, state: &mut Manager, now: u64, pid: u32, frame: &[u8]) -> bool {
    if !is_socks(frame) {
        return false;
    }
    let out = front.serve(&mut Anyone { state, now }, pid, frame);
    let _ = mk_ipc_reply(pid, out.as_ptr(), out.len());
    true
}
