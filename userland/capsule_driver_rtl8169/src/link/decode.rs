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

use crate::chip::MacVersion;

// PHYstatus (0x6C) bits, Linux enum rtl_register_content "rtl8169_PHYstatus";
// the 2500 bit is the 8125's, from Realtek's r8125 driver (rtl8125_PHYstatus),
// since mainline Linux reads the 8125's speed from the PHY instead.
const FULL_DUP: u16 = 0x01;
const LINK_STATUS: u16 = 0x02;
const SPEED_10: u16 = 0x04;
const SPEED_100: u16 = 0x08;
const SPEED_1000_FULL: u16 = 0x10;
const SPEED_2500_FULL_8125: u16 = 0x400;

/// What PHYstatus says. `speed` is in Mb/s, 0 when no speed bit is set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkState {
    pub up: bool,
    pub speed: u32,
    pub full: bool,
}

/// `raw` is PHYstatus as read: 16 bits on an 8125, 8 bits before it, where
/// bit 10 is not part of the register.
pub fn decode(raw: u16, ver: MacVersion) -> LinkState {
    let bits = if ver.is_8125() { raw } else { raw & 0xFF };
    let up = bits & LINK_STATUS != 0;
    let speed = if bits & SPEED_2500_FULL_8125 != 0 {
        2500
    } else if bits & SPEED_1000_FULL != 0 {
        1000
    } else if bits & SPEED_100 != 0 {
        100
    } else if bits & SPEED_10 != 0 {
        10
    } else {
        0
    };
    // 1000 and 2500 are full duplex only; the bit names say so.
    let full = bits & FULL_DUP != 0 || speed >= 1000;
    if up {
        LinkState { up, speed, full }
    } else {
        LinkState { up: false, speed: 0, full: false }
    }
}
