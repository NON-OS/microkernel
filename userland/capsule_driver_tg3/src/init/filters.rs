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

//! Which frames the receiver keeps, as __tg3_set_rx_mode and the end of
//! tg3_reset_hw leave it, once the PHY is set up.
//!
//! Multicast is rejected, all four hash registers zero, as for a netdev
//! with no multicast list: no client here asks to join a group. The VLAN
//! tag stays in the frame (RX_MODE_KEEP_VLAN_TAG), since the frame is all
//! a client gets, unless ASF firmware runs, which needs it stripped.

use crate::constants::literals::*;
use crate::constants::mac::*;
use crate::setup::Driver;

use super::blocks::settle;

const HASH_REGS: usize = 4;

/// tp->rx_mode as __tg3_set_rx_mode leaves it: not promiscuous, and the
/// pause bits the PHY setup chose kept.
pub fn rx_mode(current: u32, asf: bool) -> u32 {
    let base = current & !(RX_MODE_PROMISC | RX_MODE_KEEP_VLAN_TAG);
    if asf {
        base
    } else {
        base | RX_MODE_KEEP_VLAN_TAG
    }
}

/// The rules zeroed: from the last down to the four Linux leaves.
pub fn rules_zeroed(asf: bool) -> core::ops::Range<usize> {
    let limit = if asf { RCV_RULES - RCV_RULES_ASF } else { RCV_RULES };
    RCV_RULES_KEPT.min(limit)..limit
}

/*
 * SAFETY: `d.regs` carries the broker MmioMap base for BAR0 and every
 * offset is a 32-bit MAC register inside it.
 */
pub fn filters(d: &mut Driver) {
    let regs = d.regs;
    let mode = rx_mode(d.modes.rx_mode, d.modes.asf);
    unsafe {
        for n in 0..HASH_REGS {
            regs.w32(MAC_HASH_REG_0 + 4 * n, 0);
        }
        if mode != d.modes.rx_mode {
            settle(|| regs.w32f(MAC_RX_MODE, mode));
        }
        regs.w32(MAC_RCV_RULE_0, RCV_RULE_0 & RCV_RULE_DISABLE_MASK);
        regs.w32(MAC_RCV_VALUE_0, RCV_VALUE_ALL & RCV_RULE_DISABLE_MASK);
        let rule_1 = MAC_RCV_RULE_0 + RCV_RULE_STRIDE;
        regs.w32(rule_1, RCV_RULE_1 & RCV_RULE_DISABLE_MASK);
        regs.w32(rule_1 + 4, RCV_VALUE_ALL & RCV_RULE_DISABLE_MASK);
        for n in rules_zeroed(d.modes.asf) {
            regs.w32(MAC_RCV_RULE_0 + n * RCV_RULE_STRIDE, 0);
            regs.w32(MAC_RCV_VALUE_0 + n * RCV_RULE_STRIDE, 0);
        }
    }
    d.modes.rx_mode = mode;
}
