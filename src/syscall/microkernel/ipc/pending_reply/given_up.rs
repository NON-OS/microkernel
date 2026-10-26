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

//! A call that stopped waiting tells its server's table (give_up_rule.rs);
//! call/given_up.rs acts on the answer.

pub(in crate::syscall::microkernel::ipc) use super::give_up_rule::GiveUp;

use super::state::PENDING;

/// The caller of `token` on `server_pid` stops waiting.
pub(in crate::syscall::microkernel::ipc) fn give_up(server_pid: u32, token: u64) -> GiveUp {
    let mut map = PENDING.lock();
    match map.get_mut(&server_pid) {
        Some(queue) => super::give_up_rule::mark_given_up(queue.iter_mut(), token),
        None => GiveUp::Gone,
    }
}
