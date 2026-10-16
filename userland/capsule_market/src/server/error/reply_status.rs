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

pub(crate) fn reply_status(tx: &mut [u8], req: &Request, status: i32) {
    if let Some(frame) = framed(tx, req, status) {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, frame.as_ptr(), frame.len());
    }
}

/// The header and status word, written into `tx`; None when it is too short.
fn framed<'a>(tx: &'a mut [u8], req: &Request, status: i32) -> Option<&'a [u8]> {
    encode_response_header(tx, req, u32::try_from(STATUS_LEN).ok()?)?;
    write_status(tx.get_mut(RESP_HDR_LEN..)?, status)?;
    tx.get(..RESP_HDR_LEN.checked_add(STATUS_LEN)?)
}
