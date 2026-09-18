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

//! One request and its reply over the kernel's call primitive.
//!
//! `mk_ipc_call_timeout` is the only correct shape here: a bare send plus a
//! receive on inbox zero has the reply routed to the wrong inbox and it
//! never arrives. The timeout bounds what a wedged driver can cost a write
//! that is otherwise thousands of these in a row.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_ipc_call_timeout;

use super::encode_request;
use crate::error::BlkError;

/// Long enough for a first request to a driver still finishing its bring-up
/// under TCG, short enough that a dead disk fails the install in seconds.
const TIMEOUT_MS: u64 = 8000;

static REQUEST_ID: AtomicU32 = AtomicU32::new(1);

/// Returns the received byte count and the id the reply must echo.
pub fn call(
    port: u32,
    magic: u32,
    op: u16,
    body: &[u8],
    rx: &mut [u8],
) -> Result<(usize, u32), BlkError> {
    let request_id = REQUEST_ID.fetch_add(1, Ordering::Relaxed);
    let tx = encode_request(magic, op, request_id, body);
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < 0 {
        return Err(BlkError::Transport(n));
    }
    Ok((n as usize, request_id))
}
