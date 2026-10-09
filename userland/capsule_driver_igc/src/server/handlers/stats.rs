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

//! `OP_STATS`. Twelve little-endian words, in the order e1000 reports them:
//! STATUS, RCTL, TCTL, RDH, RDT, TDH, TDT for queue 0, the software RX head
//! and TX tail, both ring sizes, and RXDCTL in the last word e1000 leaves 0,
//! so a photo of the stats shows whether the receive queue is enabled.

use crate::constants::queue::{RX_DESC_COUNT, TX_DESC_COUNT};
use crate::constants::regs::{
    REG_RCTL, REG_RDH, REG_RDT, REG_RXDCTL, REG_STATUS, REG_TCTL, REG_TDH, REG_TDT,
};
use crate::protocol::{
    encode_response_header, write_status, Request, RESP_HDR_LEN, STATS_PAYLOAD_LEN, STATUS_LEN,
};
use crate::server::error::reply;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &Driver, req: &Request, tx: &mut [u8]) {
    let payload_len = STATUS_LEN as u32 + STATS_PAYLOAD_LEN as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let mut o = RESP_HDR_LEN + STATUS_LEN;
    for v in live_regs(driver) {
        tx[o..o + 4].copy_from_slice(&v.to_le_bytes());
        o += 4;
    }
    reply(sender, tx, RESP_HDR_LEN + payload_len as usize);
}

fn live_regs(driver: &Driver) -> [u32; 12] {
    let r = |off| {
        // SAFETY: `driver.regs` carries the broker MmioMap base for BAR0 and
        // every offset read here is a 32-bit register inside it.
        unsafe { driver.regs.r32(off) }
    };
    [
        r(REG_STATUS),
        r(REG_RCTL),
        r(REG_TCTL),
        r(REG_RDH),
        r(REG_RDT),
        r(REG_TDH),
        r(REG_TDT),
        driver.rx.head as u32,
        driver.tx.tail as u32,
        RX_DESC_COUNT as u32,
        TX_DESC_COUNT as u32,
        r(REG_RXDCTL),
    ]
}
