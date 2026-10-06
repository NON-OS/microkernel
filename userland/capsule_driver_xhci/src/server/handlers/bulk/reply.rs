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
//! Replies for the bulk operations: the status, a four-byte prefix, and
//! for a bulk IN the bytes that came in.
use crate::protocol::{encode_response_header, write_status, Request, RESP_HDR_LEN, STATUS_LEN};
pub(super) fn send_prefix(tx: &mut [u8], req: &Request, prefix: [u8; 4]) {
    send_with_data(tx, req, prefix, 0);
}
/// Send the reply whose `data_len` data bytes the caller already wrote
/// after the prefix.
pub(super) fn send_with_data(tx: &mut [u8], req: &Request, prefix: [u8; 4], data_len: usize) {
    let plen = STATUS_LEN + prefix.len() + data_len;
    encode_response_header(tx, req, plen as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let o = RESP_HDR_LEN + STATUS_LEN;
    tx[o..o + prefix.len()].copy_from_slice(&prefix);
    crate::server::reply::send(tx.as_ptr(), RESP_HDR_LEN + plen);
}
/// Where the data of a bulk IN reply starts in `tx`.
pub(super) const DATA_AT: usize = RESP_HDR_LEN + STATUS_LEN + 4;
