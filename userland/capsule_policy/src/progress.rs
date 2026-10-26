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

//! How a job that asks vfs between requests is getting on, for serial.
//!
//! A job whose asks keep timing out is slow, not silent: its first late ask
//! and its finish are each said once, with the passes they took. No crate
//! paths here, so the host proofs mount this file as it is.

/// The longest any one vfs call may hold the server loop: under the
/// shortest wait a client gives this service (the desktop's 64 ms).
pub const VFS_DEADLINE_MS: u64 = 25;

/// Passes made, and the pass of the first late ask (0: none yet).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Progress {
    pub passes: u32,
    pub first_late: u32,
}

/// What a pass leaves to say.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Say {
    Nothing,
    /// The first ask vfs did not answer in time, at this pass.
    FirstLate(u32),
}

impl Progress {
    pub const fn new() -> Progress {
        Progress { passes: 0, first_late: 0 }
    }

    /// One pass, `late` when its ask got no answer in time.
    pub fn pass(&mut self, late: bool) -> Say {
        self.passes = self.passes.saturating_add(1);
        if late && self.first_late == 0 {
            self.first_late = self.passes;
            return Say::FirstLate(self.passes);
        }
        Say::Nothing
    }
}
