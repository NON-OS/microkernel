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

use nonos_libc::mk_ipc_send;

use crate::protocol::{
    encode_response_header, write_status, Request, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};

/// Send the header, a zero status and the `body_len` bytes body_slot gave.
/// Nothing is sent when those do not fit `tx` or their length the wire's u32:
/// the kernel is never handed a length past the buffer.
pub(crate) fn reply_with_body(tx: &mut [u8], req: &Request, body_len: usize) {
    if let Some(frame) = framed(tx, req, body_len) {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, frame.as_ptr(), frame.len());
    }
}

fn framed<'a>(tx: &'a mut [u8], req: &Request, body_len: usize) -> Option<&'a [u8]> {
    let payload_len = STATUS_LEN.checked_add(body_len)?;
    encode_response_header(tx, req, u32::try_from(payload_len).ok()?)?;
    write_status(tx.get_mut(RESP_HDR_LEN..)?, 0)?;
    tx.get(..RESP_HDR_LEN.checked_add(payload_len)?)
}
