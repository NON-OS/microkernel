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

//! One request and its reply.

use nonos_libc::mk_ipc_call_timeout;
use nonos_policy_proto::{Header, E_OK, HDR_LEN, IPC_PAYLOAD_MAX};

const REPLY_TIMEOUT_MS: u64 = 200;

pub struct Reply<'a> {
    pub header: Header,
    pub payload: &'a [u8],
}

/// Send one request and return the reply, or `None` if it cannot be trusted.
///
pub fn call<'a>(
    port: u32,
    op: u16,
    field: u32,
    kind: u8,
    rx: &'a mut [u8; IPC_PAYLOAD_MAX],
) -> Option<Reply<'a>> {
    let mut tx = [0u8; HDR_LEN];
    Header { op, field, kind, status: 0, payload_len: 0 }.encode(&mut tx);
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        HDR_LEN,
        rx.as_mut_ptr(),
        rx.len(),
        REPLY_TIMEOUT_MS,
    );
    if n <= 0 || (n as usize) < HDR_LEN {
        return None;
    }
    let header = Header::decode(&rx[..HDR_LEN])?;
    let body_end = HDR_LEN + header.payload_len as usize;
    if header.status != E_OK || header.op != op || body_end > n as usize {
        return None;
    }
    Some(Reply { header, payload: &rx[HDR_LEN..body_end] })
}
