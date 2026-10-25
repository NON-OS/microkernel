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

//! A configuration descriptor read for the authorization gate by index,
//! through the multiplexer like a class driver's own GET_DESCRIPTOR, its
//! reply held back and read here; `None` when the device refuses the index.

use alloc::vec::Vec;

use super::mux::{status, Mux};
use crate::protocol::{
    Request, CONTROL_DATA_MAX, CONTROL_TRANSFER_REPLY_PREFIX, OP_CONTROL_TRANSFER, RESP_HDR_LEN,
    STATUS_LEN,
};
use crate::server::reply::captured;

/// GET_DESCRIPTOR(CONFIGURATION, `index`) (USB 2.0 9.4.3).
pub(super) fn config_at(
    m: &mut Mux,
    req: &Request,
    slot: u8,
    index: u8,
    tx: &mut [u8],
) -> Option<Vec<u8>> {
    let [lo, hi] = CONTROL_DATA_MAX.to_le_bytes();
    let body = [slot, 0, 0x80, 0x06, index, 0x02, 0, 0, lo, hi];
    let read = Request { op: OP_CONTROL_TRANSFER, payload_len: body.len() as u32, ..*req };
    let len = captured(|| m.inner(&read, &body, tx));
    if status(tx, len) != Some(0) {
        return None;
    }
    let at = RESP_HDR_LEN + STATUS_LEN;
    let n = u16::from_le_bytes([*tx.get(at)?, *tx.get(at + 1)?]) as usize;
    let from = at + CONTROL_TRANSFER_REPLY_PREFIX;
    tx.get(from..from + n).map(|b| b.to_vec())
}
