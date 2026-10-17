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
//! Requests as the compositor sends them, through the shipping dispatch,
//! and the reply it would read.

use nonos_libc::take_reply;

use super::rig::Rig;
use crate::protocol::{Request, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::dispatch::dispatch;

pub const OWNER: u32 = 7;

/// The status and the body after it.
pub fn ask(r: &Rig, pid: u32, op: u16, body: &[u8], now: u64) -> (i32, Vec<u8>) {
    let req = Request { op, flags: 0, request_id: 1 };
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    dispatch(&r.d, pid, req, body, now, &mut tx);
    let (to, reply) = take_reply().expect("a reply");
    assert_eq!(to, pid, "the reply goes to the sender");
    let status = i32::from_le_bytes(reply[HDR_LEN..HDR_LEN + 4].try_into().unwrap());
    (status, reply[HDR_LEN + 4..].to_vec())
}

pub fn words(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

/// A body with u64 fields at the given byte offsets.
pub fn with64(mut b: Vec<u8>, at: usize, v: u64) -> Vec<u8> {
    b.resize(b.len().max(at + 8), 0);
    b[at..at + 8].copy_from_slice(&v.to_le_bytes());
    b
}
