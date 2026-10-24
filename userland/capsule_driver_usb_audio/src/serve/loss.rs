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

//! When the device counts as gone. After a failed isoch batch, driver.xhci0
//! recovers the ring and forgets the batch, so the next request has nothing
//! to wait for and answers success at once. That one success says nothing
//! about the device: only a success that follows a success clears the
//! count, so a device that is gone still reaches LOST_AFTER failures.

/// Isoch requests failing, not counting the hollow success after each one,
/// before the device counts as gone.
pub const LOST_AFTER: u32 = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Loss {
    failing: u32,
    after_err: bool,
}

impl Loss {
    /// Record one batch's result; true once the device counts as gone.
    pub fn note(&mut self, ok: bool) -> bool {
        match (ok, self.after_err) {
            (false, _) => {
                self.failing += 1;
                self.after_err = true;
            }
            (true, true) => self.after_err = false,
            (true, false) => self.failing = 0,
        }
        self.failing >= LOST_AFTER
    }

    pub fn clear(&mut self) {
        *self = Loss::default();
    }
}
