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

//! Silicon workarounds Linux applies once the rings are set up.
//!
//! pch_spt only (e1000_configure_tx): data corruption and a transmit hang,
//! avoided with IOSFPC.RDMTS_HEX and two outstanding requests instead of
//! three in TARC0. pch_spt and later with legacy interrupts (e1000e_reset,
//! `int_mode == 0`): ungate the side clock and disable IOSF sideband clock
//! gating and requests. This driver binds no interrupt at all, which is
//! nearest to that mode.

use crate::constants::pch_bits::{
    FEXTNVM7_SIDE_CLK_UNGATE, FEXTNVM9_IOSFSB_CLKGATE_DIS, FEXTNVM9_IOSFSB_CLKREQ_DIS,
};
use crate::constants::regs::REG_TARC0;
use crate::constants::regs_pch::{REG_FEXTNVM7, REG_FEXTNVM9, REG_IOSFPC};
use crate::constants::rxtx::{RCTL_RDMTS_HEX, TARC0_CB_MULTIQ_2_REQ, TARC0_CB_MULTIQ_3_REQ};
use crate::constants::Family;
use crate::regs::Regs;

pub fn run(regs: &Regs, family: Family) {
    // SAFETY: `regs` is the broker-mapped BAR0 window; every offset here is
    // a 4-byte register inside it on the families each branch names.
    unsafe {
        if family == Family::PchSpt {
            regs.modify(REG_IOSFPC, 0, RCTL_RDMTS_HEX);
            regs.modify(REG_TARC0, TARC0_CB_MULTIQ_3_REQ, TARC0_CB_MULTIQ_2_REQ);
        }
        if family >= Family::PchSpt {
            regs.modify(REG_FEXTNVM7, 0, FEXTNVM7_SIDE_CLK_UNGATE);
            let gate = FEXTNVM9_IOSFSB_CLKGATE_DIS | FEXTNVM9_IOSFSB_CLKREQ_DIS;
            regs.modify(REG_FEXTNVM9, 0, gate);
        }
    }
}
