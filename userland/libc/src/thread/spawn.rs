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
use crate::syscall::{call_raw, N_MK_THREAD_SPAWN};

/// Start a thread in this process at `entry` with its stack pointer at
/// `stack`. The new thread's id, or a negative errno: EINVAL for an entry or
/// a stack outside the user half, ENOMEM when the kernel could not build it.
/// Nothing is passed in registers; whatever the thread needs rides its stack.
pub extern "C" fn mk_thread_spawn(entry: u64, stack: u64) -> i64 {
    call_raw(N_MK_THREAD_SPAWN, [entry, stack, 0, 0, 0, 0])
}
