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

//! `OP_STATS`. Twelve little-endian words, in the places e1000 reports its
//! own: MAC_STATUS, MAC_RX_MODE (for RCTL), MAC_TX_MODE (for TCTL), the
//! receive producer the part last wrote back and the standard producer the
//! driver last gave it (RDH, RDT), the send consumer written back and the
//! send producer given (TDH, TDT), the driver's return-ring consumer and
//! send consumer, both ring sizes, and MAC_MODE in the last word.

use crate::constants::mac::{MAC_MODE, MAC_RX_MODE, MAC_STATUS, MAC_TX_MODE};
use crate::constants::ring::{RX_STD_RING_SIZE, TX_RING_SIZE};
use crate::protocol::{
    encode_response_header, write_status, Request, RESP_HDR_LEN, STATS_PAYLOAD_LEN, STATUS_LEN,
};
use crate::server::error::reply;
use crate::setup::Driver;

pub fn handle(sender: u32, d: &Driver, req: &Request, tx: &mut [u8]) {
    let payload_len = STATUS_LEN as u32 + STATS_PAYLOAD_LEN as u32;
    encode_response_header(tx, req, payload_len);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let mut o = RESP_HDR_LEN + STATUS_LEN;
    for v in words(d) {
        tx[o..o + 4].copy_from_slice(&v.to_le_bytes());
        o += 4;
    }
    reply(sender, tx, RESP_HDR_LEN + payload_len as usize);
}

fn words(d: &Driver) -> [u32; 12] {
    let r = |off| {
        // SAFETY: `d.regs` carries the broker MmioMap base for BAR0 and
        // every offset read here is a 32-bit MAC register inside it.
        unsafe { d.regs.r32(off) }
    };
    [
        r(MAC_STATUS),
        r(MAC_RX_MODE),
        r(MAC_TX_MODE),
        d.status.rx_producer() as u32,
        d.rx.std_prod as u32,
        d.status.tx_consumer() as u32,
        d.tx.prod as u32,
        d.rx.ret_cons as u32,
        d.tx.cons as u32,
        RX_STD_RING_SIZE as u32,
        TX_RING_SIZE as u32,
        r(MAC_MODE),
    ]
}
