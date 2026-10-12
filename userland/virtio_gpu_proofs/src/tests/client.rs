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

//! The compositor's side of driver.virtio_gpu0, as the tests play it.

use nonos_libc::replies;

use crate::driver::Driver;
use crate::protocol::{Request, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::dispatch::dispatch;

/// Send `body` as `op` from `pid`: the reply it drew, from the status word.
pub fn answer(d: &Driver, pid: u32, op: u16, body: &[u8]) -> Vec<u8> {
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    dispatch(d, pid, Request { op, flags: 0, request_id: 9 }, body, &mut tx);
    let (to, reply) = replies().pop().expect("a reply");
    assert_eq!(to, pid, "answered to its sender");
    reply[HDR_LEN..].to_vec()
}

/// Send `body` as `op` from `pid`: the status word of the reply it drew.
pub fn call(d: &Driver, pid: u32, op: u16, body: &[u8]) -> i32 {
    let reply = answer(d, pid, op, body);
    i32::from_le_bytes(reply[..4].try_into().expect("a status word"))
}

/// An OP_FLUSH body: resource, then the rectangle.
pub fn flush(resource_id: u32, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
    [resource_id, x, y, w, h].iter().flat_map(|v| v.to_le_bytes()).collect()
}
