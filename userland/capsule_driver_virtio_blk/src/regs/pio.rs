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

//! Port I/O through the broker's grant. An offset that does not fit the
//! syscall's 16 bits reaches no port: a read answers zero and a write is
//! dropped, as when the broker refuses one past the grant.

use nonos_libc::{mk_pio_read, mk_pio_write};

pub fn read(grant: u64, offset: usize, width: u8) -> u32 {
    let mut value = 0u32;
    let Ok(port) = u16::try_from(offset) else {
        return 0;
    };
    if mk_pio_read(grant, port, width, &mut value) < 0 {
        return 0;
    }
    value
}

pub fn write(grant: u64, offset: usize, width: u8, value: u32) {
    if let Ok(port) = u16::try_from(offset) {
        let _ = mk_pio_write(grant, port, width, value);
    }
}
