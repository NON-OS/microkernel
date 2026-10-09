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

//! ALDPS (link-down power saving) off, as r8153_aldps_en(false) does at
//! the head of r8153_hw_phy_cfg and rtl8153_up "before updating the PHY
//! parameters". Linux turns it back on afterwards; it stays off here,
//! since it only saves power while no cable is in.

use nonos_usbnet::Bus;

use crate::r8153::fail::{at, Fail, E_TIMEDOUT};
use crate::r8153::ocp::{phy_read, phy_write, read_word, wait_until, Dev, PLA};
use crate::r8153::regs::phy::{EN_ALDPS, OCP_POWER_CFG};
use crate::r8153::regs::pla::ALDPS_STATE;

/// Linux looks 20 times, 1 to 2 ms apart, for bit 8 of PLA 0xe000.
const WAIT_MS: u64 = 40;
const ALDPS_LEFT: u16 = 0x0100;

pub fn aldps_off<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    let cfg = at("ALDPS config unread", phy_read(dev, OCP_POWER_CFG))?;
    at("ALDPS not turned off", phy_write(dev, OCP_POWER_CFG, cfg & !EN_ALDPS))?;
    let left = |d: &mut Dev<B>| Ok(read_word(d, PLA, ALDPS_STATE)? & ALDPS_LEFT != 0);
    match wait_until(dev, WAIT_MS, 1, left) {
        // Linux goes on when the bit never shows, and so does this.
        Ok(()) | Err(E_TIMEDOUT) => Ok(()),
        Err(e) => Err(("ALDPS state unread", e)),
    }
}
