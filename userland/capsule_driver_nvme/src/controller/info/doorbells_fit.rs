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

use super::info_type::ControllerInfo;
use crate::nvm::{cq_head_doorbell, IO_QID};

impl ControllerInfo {
    /// True when every doorbell the driver rings lies wholly inside the
    /// `mapped` bytes of BAR0 the broker gave it. CAP.DSTRD is the device's
    /// to choose and spaces the doorbells 4 << DSTRD bytes apart, so a stride
    /// of 15 puts the I/O completion doorbell 384 KiB in; the broker also ends
    /// the window below an MSI-X table that shares BAR0. A doorbell past the
    /// window is a store into whatever the capsule has mapped there, or a
    /// fault. The I/O completion head is the highest doorbell the driver
    /// writes (admin queue 0 sits below it), so it alone is checked.
    pub fn doorbells_fit(self, mapped: u64) -> bool {
        let last = cq_head_doorbell(IO_QID, self.doorbell_stride()) as u64;
        match last.checked_add(4) {
            Some(end) => end <= mapped,
            None => false,
        }
    }
}
