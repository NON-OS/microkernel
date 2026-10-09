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

use crate::constants::regs::{
    CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD, REG_IMR, REG_ISR,
};
use crate::setup::Driver;

use super::{mac, reset, rx_setup, tx_setup};

/*
 * Rx and Tx are enabled before RCR and TCR are written. Linux 8139too, and
 * the BSD rl and rtk drivers, all do it in this order ("Must enable Tx/Rx
 * before setting transfer thresholds!"): on silicon where those writes do
 * not take while the engines are off, RCR keeps its reset value and accepts
 * nothing.
 */
pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    reset::run(&driver.pio)?;
    driver.mac = mac::program(&driver.pio)?;
    rx_setup::program(driver)?;
    tx_setup::program(driver)?;
    driver.pio.w8(REG_CMD, CMD_RX_ENABLE | CMD_TX_ENABLE)?;
    rx_setup::configure(driver)?;
    tx_setup::configure(driver)?;
    driver.pio.w16(REG_ISR, 0xFFFF)?;
    // The driver polls and binds no line, so the part raises none: an
    // unmasked source nobody services holds a shared INTx asserted for
    // every other device on it. ISR still latches, which is all it reads.
    driver.pio.w16(REG_IMR, 0)
}
