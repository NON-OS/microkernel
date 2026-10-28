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

//! How often a device that does not answer is brought up again: at most
//! BRING_UP_TRIES times in a row, half a second apart (run.rs), ten seconds
//! as the bring-up policy's old deadline was; then the driver leaves with
//! EXIT_GAVE_UP. A device that came up and later went away starts afresh.

pub const BRING_UP_TRIES: u32 = 20;

/// Bring-ups failed in a row.
#[derive(Default)]
pub struct Tries {
    failed: u32,
}

impl Tries {
    /// One more failed: whether it was the last one allowed.
    pub fn failed(&mut self) -> bool {
        self.failed = self.failed.saturating_add(1);
        self.failed >= BRING_UP_TRIES
    }
}
