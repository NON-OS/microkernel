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

//! A thread published but never started, ended there (L77). Its process may
//! have been killed while it was built: that kill either saw it in the table
//! and counted it, or it sees the process dead here and ends at once, so it
//! never runs on tables whose devices the kill already gave back.

use super::super::types::{Pid, ProcessState};
use crate::process::nonos_core::ProcessControlBlock;
use crate::syscall::microkernel::errnos::ERRNO_NOMEM;

/// End a thread that was published but cannot run, and pass on why.
pub(super) fn unstarted(tid: Pid, why: &'static str) -> &'static str {
    crate::process::exit::teardown(tid, ERRNO_NOMEM as i32, false);
    why
}

pub(super) fn parent_ended(parent: &ProcessControlBlock) -> bool {
    matches!(*parent.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
}
