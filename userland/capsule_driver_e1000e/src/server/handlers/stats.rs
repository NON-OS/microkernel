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

//! `OP_STATS`: twelve little-endian u32s, a side-effect-free snapshot of
//! the live registers and ring cursors, in the e1000 capsule's order so the
//! same client reads both: STATUS, RCTL, TCTL, RDH, RDT, TDH, TDT, rx head,
//! tx tail, ring sizes, then CTRL_EXT where e1000 has a zero (it shows
//! FORCE_SMBUS and DRV_LOAD, the PCH bring-up's footprints).

use crate::constants::queue::{RX_DESC_COUNT, TX_DESC_COUNT};
use crate::constants::regs::{
    REG_CTRL_EXT, REG_RCTL, REG_RDH, REG_RDT, REG_STATUS, REG_TCTL, REG_TDH, REG_TDT,
};
use crate::protocol::{
    encode_response_header, write_status, Request, RESP_HDR_LEN, STATS_PAYLOAD_LEN, STATUS_LEN,
};
use crate::server::error::reply;
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &Driver, req: &Request, tx: &mut [u8]) {
    let payload_len = (STATUS_LEN + STATS_PAYLOAD_LEN) as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let base = RESP_HDR_LEN + STATUS_LEN;
    for (i, v) in live(driver).iter().enumerate() {
        tx[base + i * 4..base + i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    reply(sender, tx, RESP_HDR_LEN + payload_len as usize);
}

fn live(d: &Driver) -> [u32; 12] {
    // SAFETY: `d.regs` is the broker-mapped BAR0 window; every offset read
    // is a 4-byte register inside it, and none of them clears on read.
    unsafe {
        [
            d.regs.r32(REG_STATUS),
            d.regs.r32(REG_RCTL),
            d.regs.r32(REG_TCTL),
            d.regs.r32(REG_RDH),
            d.regs.r32(REG_RDT),
            d.regs.r32(REG_TDH),
            d.regs.r32(REG_TDT),
            d.rx.head as u32,
            d.tx.tail as u32,
            RX_DESC_COUNT as u32,
            TX_DESC_COUNT as u32,
            d.regs.r32(REG_CTRL_EXT),
        ]
    }
}
