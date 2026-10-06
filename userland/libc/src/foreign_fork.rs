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

//! Forking a guest: a second process holding its register state, with zero
//! in its return register, on its parent's stack or one the caller names.

use crate::syscall::{call_raw, N_MK_FOREIGN_FORK};

/// A second process holding a guest's register state, with zero in its return
/// register.
pub fn mk_foreign_fork(pid: u32) -> i64 {
    mk_foreign_fork_at(pid, 0)
}

/// A fork whose child starts on `rsp` rather than its parent's stack pointer,
/// as a clone that names a stack asks; zero keeps the parent's.
pub fn mk_foreign_fork_at(pid: u32, rsp: u64) -> i64 {
    call_raw(N_MK_FOREIGN_FORK, [pid as u64, rsp, 0, 0, 0, 0])
}
