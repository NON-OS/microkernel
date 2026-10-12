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

//! One call to capsule_nox_settle, on settle_proto's wire, found by name
//! each time. The reply is believed only for this call.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call_timeout, mk_time_millis};
use nonos_payment_proto::{EAGAIN, EINVAL, EIO};
use nonos_settle_proto::{reply_body, request, ReplyError, HDR_LEN, SERVICE, STATUS_LEN};

use crate::clients::port_of;

/// The reply body, at most `max` bytes, or settle's status.
pub(super) fn settle_call(
    op: u16,
    body: &[u8],
    max: usize,
    timeout_ms: u64,
) -> Result<Vec<u8>, i32> {
    let port = port_of(SERVICE).ok_or(EAGAIN)?;
    let id = mk_time_millis() as u32;
    let tx = request(op, id, body).ok_or(EINVAL)?;
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN + max];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        timeout_ms,
    );
    if n <= 0 {
        return Err(EAGAIN);
    }
    match reply_body(&rx[..n as usize], op, id) {
        Ok(b) => Ok(b.to_vec()),
        Err(ReplyError::Status(s)) => Err(s),
        Err(_) => Err(EIO),
    }
}
