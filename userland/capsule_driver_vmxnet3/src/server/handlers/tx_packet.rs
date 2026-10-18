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

//! `OP_TX_PACKET`. A frame is answered once it is queued, not once it is on
//! the wire: completion is what `reclaim` observes, and a ring with no free
//! slot says so with `E_AGAIN` rather than holding the caller.

use crate::constants::ring::BUFFER_LEN;
use crate::constants::{MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
use crate::protocol::{Request, E_AGAIN, E_INVAL};
use crate::server::error::reply_with_status;
use crate::server::handlers::tx_stage::stage;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
    if body.len() < MIN_ETHERNET_FRAME || body.len() > MAX_ETHERNET_FRAME {
        return reply_with_status(sender, tx, req, E_INVAL);
    }
    driver.tx.reclaim();
    if driver.tx.full() {
        return reply_with_status(sender, tx, req, E_AGAIN);
    }
    let va = driver.tx.buffer_va(driver.tx.fill);
    // SAFETY: slot `fill` of the transmit pool is BUFFER_LEN bytes, the
    // device does not own it (`full` said so), and the request is elsewhere.
    let slot = unsafe { core::slice::from_raw_parts_mut(va as *mut u8, BUFFER_LEN) };
    let len = stage(slot, body);
    let next = driver.tx.post(len as u16);
    driver.regs.tx_prod(next);
    reply_with_status(sender, tx, req, 0)
}
