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

//! `OP_RX_PACKET`: one frame, for a caller that does not batch. Dropped and
//! stray completions are passed over, at most one ring's worth.

use crate::constants::ring::RX_COMP;
use crate::protocol::{
    encode_response_header, write_status, Request, E_AGAIN, RESP_HDR_LEN, RX_PAYLOAD_PREFIX_LEN,
    STATUS_LEN,
};
use crate::queue::Completion;
use crate::server::error::{reply, reply_with_status};
use crate::setup::Driver;

pub fn handle(sender: u32, driver: &mut Driver, req: &Request, tx: &mut [u8]) -> bool {
    for _ in 0..RX_COMP {
        let (idx, len) = match driver.rx.consume() {
            None => break,
            Some(Completion::Stray) => continue,
            Some(Completion::Dropped) => {
                let prod = driver.rx.post();
                driver.regs.rx_prod(prod, 0);
                continue;
            }
            Some(Completion::Frame { idx, len }) => (idx, len as usize),
        };
        let at = RESP_HDR_LEN + STATUS_LEN;
        let frame_at = at + RX_PAYLOAD_PREFIX_LEN;
        let n = len.min(tx.len().saturating_sub(frame_at));
        // SAFETY: slot `idx` holds `len` bytes the device wrote and completed;
        // it is the driver's until the post below hands a descriptor back.
        let src = unsafe { core::slice::from_raw_parts(driver.rx.buffer_va(idx) as *const u8, n) };
        tx[frame_at..frame_at + n].copy_from_slice(src);
        let prod = driver.rx.post();
        driver.regs.rx_prod(prod, 0);
        encode_response_header(tx, req, (STATUS_LEN + RX_PAYLOAD_PREFIX_LEN + n) as u32);
        write_status(&mut tx[RESP_HDR_LEN..], 0);
        tx[at..frame_at].copy_from_slice(&(n as u32).to_le_bytes());
        return reply(sender, tx, frame_at + n);
    }
    reply_with_status(sender, tx, req, E_AGAIN)
}
