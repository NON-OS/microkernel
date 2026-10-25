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

//! A root port's status in the form the host wire carries: xHCI's PORTSC
//! layout. Only a port holding a high-speed device is this host's to
//! report; one handed to the companion, or empty, reads as nothing here.
//! Connected (bit 0), enabled (bit 1), speed 3 (high) in 13:10, and the
//! connect change (bit 17). Pure.

use crate::facts::port::{PORT_CCS, PORT_CSC, PORT_OWNER, PORT_PE};

pub const SPEED_HIGH: u8 = 3;

pub const fn wire_portsc(raw: u32, high: bool) -> u32 {
    if !high || raw & PORT_OWNER != 0 || raw & PORT_CCS == 0 {
        return 0;
    }
    let mut w = 1 | (SPEED_HIGH as u32) << 10;
    if raw & PORT_PE != 0 {
        w |= 1 << 1;
    }
    if raw & PORT_CSC != 0 {
        w |= 1 << 17;
    }
    w
}
