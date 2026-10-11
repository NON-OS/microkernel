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

//! The pause the two ends agreed on (mii_resolve_flowctrl_fdx), pure.

use crate::constants::phy::{ADVERTISE_PAUSE_ASYM, ADVERTISE_PAUSE_CAP};

/// FLOW_CTRL_TX and FLOW_CTRL_RX (include/uapi/linux/mii.h).
pub const FLOW_CTRL_TX: u8 = 0x01;
pub const FLOW_CTRL_RX: u8 = 0x02;

pub fn flow_control(lcladv: u32, rmtadv: u32) -> u8 {
    if lcladv & rmtadv & ADVERTISE_PAUSE_CAP != 0 {
        FLOW_CTRL_TX | FLOW_CTRL_RX
    } else if lcladv & rmtadv & ADVERTISE_PAUSE_ASYM != 0 {
        if lcladv & ADVERTISE_PAUSE_CAP != 0 {
            FLOW_CTRL_RX
        } else if rmtadv & ADVERTISE_PAUSE_CAP != 0 {
            FLOW_CTRL_TX
        } else {
            0
        }
    } else {
        0
    }
}
