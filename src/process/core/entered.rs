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

//! The state an entry path leaves on the thread it enters, or gives back
//! when it entered nothing. A kill from another CPU can land after the
//! switch asked `is_dead` and before the arch layer stores Running: a plain
//! store put the zombie back to Running, no tick then saw it dead, and it
//! ran on with its group's devices held (L77c). Each store here is made
//! under the state lock and never over a dead state.

use super::{ProcessControlBlock, ProcessState};

impl ProcessControlBlock {
    /// Running, unless killed: false then, nothing stored, and the caller
    /// enters nothing. Asked before the entry loads any of the thread's
    /// stack or tables, so a refusal leaves the CPU as it was.
    pub(crate) fn mark_running(&self) -> bool {
        self.live_to(ProcessState::Running)
    }

    /// Ready again after an entry that loaded nothing, unless killed. The
    /// x86_64 entry has no such path: it parks or retries instead.
    #[cfg(not(target_arch = "x86_64"))]
    pub(crate) fn mark_ready(&self) {
        let _ = self.live_to(ProcessState::Ready);
    }

    /// Terminated(-1) for an entry that cannot go on, unless killed: the
    /// kill's own code stands for the parent's wait (L77d).
    pub(crate) fn mark_failed(&self) {
        let _ = self.live_to(ProcessState::Terminated(-1));
    }

    fn live_to(&self, to: ProcessState) -> bool {
        let mut state = self.state.lock();
        let dead = matches!(*state, ProcessState::Zombie(_) | ProcessState::Terminated(_));
        if !dead {
            *state = to;
        }
        !dead
    }
}
