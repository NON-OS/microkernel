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

//! STATUS decoded the way igc_check_for_copper_link reads link up (LU) and
//! igc_get_speed_and_duplex_copper reads speed and duplex. The I225/I226
//! sets SPEED_1000 at both 1 and 2.5 Gb/s and tells them apart with
//! SPEED_2500; SPEED_2500 without SPEED_1000 is read as Linux reads it.

use crate::constants::ctrl::{
    STATUS_FD, STATUS_LU, STATUS_SPEED_100, STATUS_SPEED_1000, STATUS_SPEED_2500,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LinkState {
    pub up: bool,
    pub mbps: u16,
    pub full: bool,
}

pub fn decode(status: u32) -> LinkState {
    let mbps = if status & STATUS_SPEED_1000 != 0 {
        if status & STATUS_SPEED_2500 != 0 {
            2500
        } else {
            1000
        }
    } else if status & STATUS_SPEED_100 != 0 {
        100
    } else {
        10
    };
    LinkState { up: status & STATUS_LU != 0, mbps, full: status & STATUS_FD != 0 }
}

/// Whether `now` is worth a line after `before`. Speed and duplex bits mean
/// nothing while the link is down, so two downs are the same state.
pub fn changed(before: Option<LinkState>, now: LinkState) -> bool {
    match before {
        None => true,
        Some(b) if !b.up && !now.up => false,
        Some(b) => b != now,
    }
}
