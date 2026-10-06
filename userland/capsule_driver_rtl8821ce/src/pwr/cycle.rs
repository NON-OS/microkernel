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

//! The power-off half of rtw88's 8821C power switch, and the first step of its
//! power-on, ported from rtw8821c.c (Linux, GPL) with only the entries whose
//! interface mask includes PCIe. A warm reboot leaves the card powered with the
//! last boot's MAC, DMA and firmware state; rtw88 `rtw_mac_power_switch`
//! (mac.c:272) runs these before powering on again, and so does this driver.

use super::command::PwrCmd;

/// `REG_CR` reads this byte only on a MAC that is powered down
/// (mac.c:291, `rtw_read8(rtwdev, REG_CR) == 0xea`).
pub const CR_UNPOWERED: u8 = 0xEA;

/// `card_disable_flow_8821c`: `trans_act_to_cardemu_8821c` (rtw8821c.c:1380)
/// then `trans_cardemu_to_carddis_8821c` (rtw8821c.c:1438), PCIe entries only.
pub const CARD_DISABLE: &[PwrCmd] = &[
    PwrCmd::write(0x0093, 0x08, 0x00),
    PwrCmd::write(0x001F, 0xFF, 0x00),
    PwrCmd::write(0x0049, 0x02, 0x00),
    PwrCmd::write(0x0006, 0x01, 0x01),
    PwrCmd::write(0x0002, 0x02, 0x00),
    PwrCmd::write(0x0005, 0x02, 0x02),
    PwrCmd::poll(0x0005, 0x02, 0x00), // the MAC has left the active state
    PwrCmd::write(0x0020, 0x08, 0x00),
    // card emulation to card disable
    PwrCmd::write(0x0067, 0x20, 0x00),
    PwrCmd::write(0x0005, 0x04, 0x04),
    PwrCmd::write(0x0081, 0xC0, 0x00),
    PwrCmd::write(0x0090, 0x02, 0x00),
    PwrCmd::end(),
];

/// `trans_carddis_to_cardemu_8821c` (rtw8821c.c:1238), PCIe entries only: the
/// first table of rtw88's power-on, needed once the card was powered off.
pub const CARD_EMULATE: &[PwrCmd] = &[
    PwrCmd::write(0x0005, 0x98, 0x00),
    PwrCmd::write(0x0300, 0xFF, 0x00),
    PwrCmd::write(0x0301, 0xFF, 0x00),
    PwrCmd::end(),
];
