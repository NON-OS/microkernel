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

//! The frames the compositor sends, laid out byte for byte as its
//! `build_request` lays them out (userland/compositor/src/gfx_client/wire),
//! and the bodies of its transfer, set_scanout and flush.

use crate::serve::{answer, Reply, PRIMARY_RESOURCE_ID, REPLY_LEN};
use crate::setup::Driver;

pub fn frame(op: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    let mut f = 0x4E56_4750u32.to_le_bytes().to_vec();
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0; 4]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}

/// `msg` from `sender`, answered: the reply's status, and the reply.
pub fn answered(d: &mut Driver, sender: u32, msg: &[u8]) -> (i32, Vec<u8>) {
    let mut tx: Reply = [0xAA; REPLY_LEN];
    let n = answer(d, sender, msg, &mut tx);
    (status_of(&tx[..n]), tx[..n].to_vec())
}

/// The status a reply carries after its header.
pub fn status_of(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[20..24].try_into().expect("a reply has a status"))
}

/// `op` with `body` from `sender`, answered.
pub fn ask(d: &mut Driver, sender: u32, op: u16, body: &[u8]) -> (i32, Vec<u8>) {
    answered(d, sender, &frame(op, 9, body))
}

pub fn fields(f: &[u32]) -> Vec<u8> {
    f.iter().flat_map(|v| v.to_le_bytes()).collect()
}

pub fn flush_body(x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
    fields(&[PRIMARY_RESOURCE_ID, x, y, w, h])
}

pub fn transfer_body(x: u32, y: u32, w: u32, h: u32, offset: u64) -> Vec<u8> {
    let mut b = fields(&[PRIMARY_RESOURCE_ID, x, y, w, h, 0]);
    b.extend_from_slice(&offset.to_le_bytes());
    b
}

pub fn scanout_body(x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
    fields(&[0, PRIMARY_RESOURCE_ID, x, y, w, h])
}
