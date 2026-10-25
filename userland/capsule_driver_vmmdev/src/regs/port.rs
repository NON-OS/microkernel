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

//! The request port, through the broker's port grant. The kernel performs
//! the `out`; VirtualBox carries the request out within it.

use nonos_libc::mk_pio_write;

#[derive(Clone, Copy)]
pub struct Port {
    grant: u64,
}

impl Port {
    pub const fn new(grant: u64) -> Self {
        Self { grant }
    }

    pub fn write32(&self, off: u16, v: u32) {
        let _ = mk_pio_write(self.grant, off, 4, v);
    }
}
