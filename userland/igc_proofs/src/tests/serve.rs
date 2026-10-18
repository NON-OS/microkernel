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

//! Requests as the stack frames them, the server loop run over them until
//! every one has been answered, and buffer pools for a driver's rings, so
//! the handlers' copies land in host memory.

use std::panic::{catch_unwind, AssertUnwindSafe};

use nonos_libc::{line_up_requests, replies, SERVED};

use crate::constants::queue::{RX_BUFFER_POOL_BYTES, TX_BUFFER_POOL_BYTES};
use crate::protocol::{encode_response_header, Request, HDR_LEN};
use crate::server::runner::run;
use crate::setup::Driver;

/// A request header for `op` claiming `payload_len` bytes, then `body`. A
/// request header has the reply's layout, so the reply encoder writes it.
pub fn request(op: u16, body: &[u8], payload_len: u32) -> Vec<u8> {
    let mut m = vec![0u8; HDR_LEN];
    let req = Request { op, flags: 0, request_id: 0x0102_0304, payload_len };
    encode_response_header(&mut m, &req, payload_len);
    m.extend_from_slice(body);
    m
}

/// A well-formed request: `payload_len` is the body's length.
pub fn ask(op: u16, body: &[u8]) -> (u32, Vec<u8>) {
    (7, request(op, body, body.len() as u32))
}

/// Run the server loop over `requests` and return every reply it sent.
pub fn serve(driver: &mut Driver, requests: &[(u32, Vec<u8>)]) -> Vec<(u32, Vec<u8>)> {
    line_up_requests(requests);
    let out = catch_unwind(AssertUnwindSafe(|| run(driver)));
    let why = out.expect_err("the loop never returns").downcast::<String>().expect("a message");
    assert_eq!(*why, SERVED, "the loop stopped early");
    replies()
}

/// The status a reply carries after its header.
pub fn status(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[HDR_LEN..HDR_LEN + 4].try_into().expect("a status"))
}

/// Both rings' buffer pools, which `d` copies through while they live.
pub fn pools(d: &mut Driver) -> (Vec<u8>, Vec<u8>) {
    let (mut rx, mut tx) = (vec![0; RX_BUFFER_POOL_BYTES], vec![0; TX_BUFFER_POOL_BYTES]);
    (d.rx.buffer_user_va, d.tx.buffer_user_va) = (rx.as_mut_ptr() as u64, tx.as_mut_ptr() as u64);
    (rx, tx)
}
