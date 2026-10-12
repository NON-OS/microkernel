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
//! layout, which every class driver reads. Connected (bit 0), enabled
//! (bit 1), the speed id in 13:10 (1 full speed, 2 low speed) and the
//! connect change (bit 17). Pure.

use crate::facts::io::{PORT_CCS, PORT_CSC, PORT_LSDA, PORT_PE};

pub const SPEED_FULL: u8 = 1;
pub const SPEED_LOW: u8 = 2;

pub const fn speed_id(low_speed: bool) -> u8 {
    if low_speed {
        SPEED_LOW
    } else {
        SPEED_FULL
    }
}

pub const fn wire_portsc(raw: u16) -> u32 {
    let mut w = 0;
    if raw & PORT_CCS != 0 {
        w |= 1 | (speed_id(raw & PORT_LSDA != 0) as u32) << 10;
    }
    if raw & PORT_PE != 0 {
        w |= 1 << 1;
    }
    if raw & PORT_CSC != 0 {
        w |= 1 << 17;
    }
    w
}
