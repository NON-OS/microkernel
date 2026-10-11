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

//! What the MAC is told of the link: its status cleared before a look, the
//! port mode and duplex after one (tg3_clear_mac_status, the tail of
//! tg3_setup_copper_phy), and the pause bits (tg3_setup_flow_control).

use crate::constants::mac::*;
use crate::init::wait;
use crate::link::LinkState;
use crate::regs::Regs;
use crate::setup::Driver;

use super::pause::{flow_control, FLOW_CTRL_RX, FLOW_CTRL_TX};

/*
 * SAFETY for every register access here: the Regs carry the broker MmioMap
 * base for BAR0 and each offset is a 32-bit MAC register inside it.
 */
pub fn clear_status(regs: &Regs) {
    unsafe {
        regs.w32(MAC_EVENT, 0);
        let changed = MAC_STATUS_SYNC_CHANGED
            | MAC_STATUS_CFG_CHANGED
            | MAC_STATUS_MI_COMPLETION
            | MAC_STATUS_LNKSTATE_CHANGED;
        regs.w32f(MAC_STATUS, changed);
    }
    wait::hold(1);
}

pub fn set_port(d: &mut Driver, state: LinkState) {
    let m = &mut d.modes.mac_mode;
    *m &= !(MAC_MODE_PORT_MODE_MASK | MAC_MODE_HALF_DUPLEX);
    let slow = state.up && matches!(state.mbps, 10 | 100);
    *m |= if slow { MAC_MODE_PORT_MODE_MII } else { MAC_MODE_PORT_MODE_GMII };
    // An unknown duplex is not half: tg3 sets the bit only for DUPLEX_HALF.
    if state.mbps != 0 && !state.full {
        *m |= MAC_MODE_HALF_DUPLEX;
    }
    unsafe { d.regs.w32f(MAC_MODE, *m) };
    wait::hold(1);
    unsafe { d.regs.w32f(MAC_EVENT, MAC_EVENT_LNKSTATE_CHANGED) };
    wait::hold(1);
}

/// Autonegotiation with pause autoneg on, as tg3 runs it on copper.
pub fn pause(d: &mut Driver, lcladv: u32, rmtadv: u32) {
    let fc = flow_control(lcladv, rmtadv);
    let (rx, tx) = (d.modes.rx_mode, d.modes.tx_mode);
    let set = |word: u32, bit: u32, on: bool| if on { word | bit } else { word & !bit };
    d.modes.rx_mode = set(rx, RX_MODE_FLOW_CTRL_ENABLE, fc & FLOW_CTRL_RX != 0);
    d.modes.tx_mode = set(tx, TX_MODE_FLOW_CTRL_ENABLE, fc & FLOW_CTRL_TX != 0);
    if d.modes.rx_mode != rx {
        unsafe { d.regs.w32f(MAC_RX_MODE, d.modes.rx_mode) };
    }
    if d.modes.tx_mode != tx {
        unsafe { d.regs.w32f(MAC_TX_MODE, d.modes.tx_mode) };
    }
}
