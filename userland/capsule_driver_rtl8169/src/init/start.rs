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
use crate::hw::start;
use crate::regmap::mask_and_ack;
use crate::setup::{command, Driver};

use super::{mac, rx_setup, tx_setup};

/*
 * A reset part put on the air: the station address, both rings named, Bus
 * Master on, and only then TE|RE. Bus Master goes on after RxDescAddr and
 * TxDescAddr are written, so a part an earlier owner left pointing at its
 * own, since freed, rings is never mastered.
 *
 * TE|RE go on before RxConfig and TxConfig are written, as Linux rtl_hw_start
 * does on every chip: on the 8169 and the 8168B-F an RxConfig written with
 * the receiver off can be lost, the accept bits never take, and nothing is
 * received.
 */
pub fn run(driver: &Driver) -> Result<(), &'static str> {
    let (regs, ver) = (&driver.regs, driver.chip.ver);
    mac::program(regs, &driver.mac)?;
    start(regs, ver, || {
        rx_setup::program(regs, &driver.rx);
        tx_setup::program(regs, &driver.tx);
    });
    command(driver.device_id, driver.claim_epoch, true)?;
    // SAFETY: ChipCmd (0x37) lies inside every mapped window, and the driver
    // owns the part.
    unsafe { regs.w8(REG_CMD, CMD_RX_ENABLE | CMD_TX_ENABLE) };
    rx_setup::configure(regs, ver);
    tx_setup::configure(regs, ver);
    // The driver polls and binds no line, so the part raises none: an
    // unmasked source nobody services holds a shared INTx asserted for
    // every other device on it. ISR still latches, which is all it reads.
    mask_and_ack(regs, ver);
    Ok(())
}
