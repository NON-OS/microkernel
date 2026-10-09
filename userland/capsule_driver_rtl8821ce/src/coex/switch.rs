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

//! The antenna switch, rtw8821c_coex_cfg_ant_switch (rtw8821c.c:837) under
//! baseband software control, with the board's front-end option read as
//! rtw8821c_coex_cfg_rfe_type (rtw8821c.c:947) reads it.

use super::init::{set32, set8};
use crate::regs::Mmio;

/// Where the switch points (COEX_SWITCH_TO_*).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AntSwitch {
    Wlg,
    Wla,
    WlgBt,
}

const REG_LED_CFG: usize = 0x004C;
const REG_RFE_CTRL8: usize = 0x0CB4;
const REG_CTRL_TYPE: usize = 0x0067;

/// The value written to REG_RFE_CTRL8 bits 31:28 for front-end `rfe`, and the
/// position after rtw88's BTG override. Pure, so the proofs check every option.
pub fn switch_to(rfe: u8, pos: AntSwitch) -> (AntSwitch, u32) {
    let wlg_at_btg = matches!(rfe, 2 | 10 | 7 | 15 | 4 | 12);
    let inverse = matches!(rfe, 3 | 11 | 4 | 12);
    let pos = match (wlg_at_btg, inverse) {
        (true, true) => AntSwitch::Wla,
        (true, false) => AntSwitch::WlgBt,
        (false, _) => pos,
    };
    let regval = match pos {
        AntSwitch::WlgBt if rfe != 4 && rfe != 2 => 0x3,
        AntSwitch::WlgBt | AntSwitch::Wlg => if inverse { 0x1 } else { 0x2 },
        AntSwitch::Wla => if inverse { 0x2 } else { 0x1 },
    };
    (pos, regval)
}

/// Whether the board has an antenna switch: rtw88 leaves the pins alone on
/// the two-antenna boards without one (rfe 5, 6, 13, 14; ant_switch_exist).
pub fn has_switch(rfe: u8) -> bool {
    !matches!(rfe, 5 | 6 | 13 | 14)
}

/// COEX_SWITCH_CTRL_BY_BBSW: DPDT driven from RFE_ctrl8/9 by the baseband.
pub(super) fn by_baseband<M: Mmio>(mmio: &M, rfe: u8, pos: AntSwitch) {
    if !has_switch(rfe) {
        return;
    }
    let (_, regval) = switch_to(rfe, pos);
    // BIT_DPDT_SEL_EN off, BIT_DPDT_WL_SEL on.
    set32(mmio, REG_LED_CFG, 1 << 24, 1 << 23);
    // BIT_MASK_RFE_SEL89 = DPDT_CTRL_PIN.
    mmio.write8(REG_RFE_CTRL8, 0x77);
    let v = mmio.read32(REG_RFE_CTRL8);
    mmio.write32(REG_RFE_CTRL8, (v & 0x0FFF_FFFF) | (regval << 28));
    // BIT_CTRL_TYPE1 | BIT_CTRL_TYPE2: not controlled by BT.
    set8(mmio, REG_CTRL_TYPE, (1 << 5) | (1 << 4));
}
