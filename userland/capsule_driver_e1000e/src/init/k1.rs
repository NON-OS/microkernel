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

//! Linux e1000_reconfigure_k1_params for pch_mtp and later: move the
//! Kumeran K1 power-down state from P0s to P1 and lengthen the K1 exit
//! timeout in the PHY. Earlier families only touch K1 when a user flag asks,
//! which this driver has no use for, so they are left as reset leaves them.
//! The caller holds the software flag.

use crate::constants::pch_bits::{FEXTNVM12_PHYPD_CTRL_MASK, FEXTNVM12_PHYPD_CTRL_P1};
use crate::constants::phy::{
    I217_PHY_TIMEOUTS, I217_PHY_TIMEOUTS_K1_EXIT_TO, I217_PHY_TIMEOUTS_K1_EXIT_TO_MASK,
};
use crate::constants::regs_pch::REG_FEXTNVM12;
use crate::constants::timeouts::SHORT_MS;
use crate::constants::Family;
use crate::phy::access::{read, write};
use crate::regs::Regs;
use crate::wait::sleep_ms;

pub fn reconfigure(regs: &Regs, family: Family) -> Result<(), &'static str> {
    if family < Family::PchMtp {
        return Ok(());
    }
    // SAFETY: `regs` is the broker-mapped BAR0 window; FEXTNVM12 is a 4-byte
    // register inside it on pch_mtp and later.
    unsafe { regs.modify(REG_FEXTNVM12, FEXTNVM12_PHYPD_CTRL_MASK, FEXTNVM12_PHYPD_CTRL_P1) };
    // Linux lets the interface settle 1 ms.
    sleep_ms(SHORT_MS);
    let t = read(regs, family, I217_PHY_TIMEOUTS).map_err(|_| "K1 timeout read failed")?;
    let t = (t & !I217_PHY_TIMEOUTS_K1_EXIT_TO_MASK) | I217_PHY_TIMEOUTS_K1_EXIT_TO;
    write(regs, family, I217_PHY_TIMEOUTS, t).map_err(|_| "K1 timeout write failed")
}
