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

//! Walk `tx_dirty` over the descriptors the part has finished with.
//!
//! The part works through TSD0-3 strictly in order with a pointer of its own,
//! so the driver has to move with it: a descriptor is finished once its status
//! says the frame went (TOK), went after an underrun re-fetch (TUN), or was
//! given up on (TABT), and each of those moves the part on to the next one.
//! Staying on a slot the part has left made every later send wait on a
//! descriptor it would never look at again. An abort also halts the
//! transmitter until TCR.CLRABT is written, as Linux 8139too does.

use crate::constants::dma::TX_SLOT_COUNT;
use crate::constants::regs::{
    REG_TCR, REG_TXSTATUS0, TCR_CLEAR_ABORT, TX_STATUS_ABORT, TX_STATUS_OK, TX_STATUS_UNDERRUN,
};
use crate::init::TCR;
use crate::setup::Driver;

pub fn reclaim(driver: &mut Driver) -> Result<(), &'static str> {
    while driver.tx_dirty != driver.tx_cur {
        let idx = driver.tx_dirty % TX_SLOT_COUNT;
        let status = driver.pio.r32(REG_TXSTATUS0 + (idx as u16 * 4))?;
        if status & (TX_STATUS_OK | TX_STATUS_UNDERRUN | TX_STATUS_ABORT) == 0 {
            break;
        }
        if status & TX_STATUS_ABORT != 0 {
            driver.pio.w32(REG_TCR, TCR | TCR_CLEAR_ABORT)?;
        }
        driver.tx_dirty = driver.tx_dirty.wrapping_add(1);
    }
    Ok(())
}

/// Whether every slot holds a frame the part has not finished with.
pub fn full(driver: &Driver) -> bool {
    driver.tx_cur.wrapping_sub(driver.tx_dirty) >= TX_SLOT_COUNT
}
