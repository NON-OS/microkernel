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

//! rtw88's coexistence power-on and Wi-Fi-only init (coex.c:2763 and 2708,
//! COEX_SET_ANT_WONLY at coex.c:1347): grant the antenna to Wi-Fi in software,
//! keep Bluetooth's grant low, make Wi-Fi the owner of the path control and
//! point the switch at the Wi-Fi 2.4 GHz port. Bluetooth is told Wi-Fi is on.

use super::indirect::write_field;
use super::init::{cfg_init, gnt_debug, set8};
use super::switch::{by_baseband, AntSwitch};
use crate::regs::Mmio;

/// LTE_COEX_CTRL, the indirect register holding both grants.
const LTE_COEX_CTRL: u16 = 0x38;
const GNT_SW_LOW: u32 = 0x1;
const GNT_SW_HIGH: u32 = 0x3;

/// Hand the antenna to Wi-Fi. `false` when the indirect window never answered,
/// so the caller can say so; the rest is plain register writes.
pub fn take_antenna<M: Mmio>(mmio: &M, rfe: u8) -> bool {
    gnt_debug(mmio);
    cfg_init(mmio);
    // Tx response, beacon and beacon queue at high priority (wl_pri_mask).
    set8(mmio, 0x06CC, 1 << 3);
    set8(mmio, 0x06CC, 1 << 4);
    set8(mmio, 0x06CC + 3, 1 << 3);
    // GNT_BT software low, GNT_WL software high.
    let ok = write_field(mmio, LTE_COEX_CTRL, 0xC000, GNT_SW_LOW)
        & write_field(mmio, LTE_COEX_CTRL, 0x0C00, GNT_SW_LOW)
        & write_field(mmio, LTE_COEX_CTRL, 0x3000, GNT_SW_HIGH)
        & write_field(mmio, LTE_COEX_CTRL, 0x0300, GNT_SW_HIGH);
    // Path control owner Wi-Fi: REG_SYS_SDIO_CTRL+3 |= BIT_LTE_MUX_CTRL_PATH>>24.
    set8(mmio, 0x0070 + 3, 1 << 2);
    by_baseband(mmio, rfe, AntSwitch::Wlg);
    // Scoreboard: Wi-Fi active and on (rtw_coex_write_scbd), with BIT_BT_INT_EN.
    mmio.write16(0x00AA, 0x8000 | 0x0003);
    ok
}
