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

//! What the broker granted, given back when the driver lets go: the
//! register mapping, then the device itself.

use nonos_libc::{mk_device_release, mk_mmio_unmap};

pub struct Handles {
    pub device_id: u64,
    /// Zero until the BAR is mapped; unmapping zero is refused harmlessly.
    pub mmio_grant: u64,
}

impl Drop for Handles {
    fn drop(&mut self) {
        let _ = mk_mmio_unmap(self.mmio_grant);
        let _ = mk_device_release(self.device_id);
    }
}
