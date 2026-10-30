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

//! The framebuffer wiped at bring-up: the firmware's framebuffer
//! outlives every driver, and what a dead one left there (a client's
//! pixels among it) is not shown again by the next.

use super::fb::Fb;

impl Fb {
    /// Every pixel of every row zeroed.
    pub fn wipe(&self) {
        for y in 0..self.height as usize {
            // SAFETY: Fb::new held every row's pixels inside the mapping.
            unsafe {
                core::ptr::write_bytes(
                    self.base.add(y * self.stride as usize),
                    0,
                    self.width as usize * 4,
                )
            };
        }
    }
}
