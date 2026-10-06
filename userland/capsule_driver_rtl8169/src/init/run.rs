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

use crate::constants::regs::{CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD};
use crate::hw::{initialize, start};
use crate::regmap::mask_and_ack;
use crate::setup::Driver;

use super::{mac, reset, rx_setup, tx_setup};

/*
 * TE|RE go on before RxConfig and TxConfig are written, as Linux rtl_hw_start
 * does on every chip: on the 8169 and the 8168B-F an RxConfig written with
 * the receiver off can be lost, the accept bits never take, and nothing is
 * received. The station address is still programmed first, so the part is
 * never enabled under the factory one.
 */
pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    initialize(&driver.regs, driver.chip.ver);
    reset::run(&driver.regs, driver.chip.ver)?;
    driver.mac = mac::program(&driver.regs)?;
    start(&driver.regs, driver.chip.ver, || {
        rx_setup::program(&driver.regs, &driver.rx);
        tx_setup::program(&driver.regs, &driver.tx);
    });
    unsafe {
        driver.regs.w8(REG_CMD, CMD_RX_ENABLE | CMD_TX_ENABLE);
    }
    rx_setup::configure(&driver.regs, driver.chip.ver);
    tx_setup::configure(&driver.regs, driver.chip.ver);
    // The driver polls and binds no line, so the part raises none: an
    // unmasked source nobody services holds a shared INTx asserted for
    // every other device on it. ISR still latches, which is all it reads.
    mask_and_ack(&driver.regs, driver.chip.ver);
    Ok(())
}
