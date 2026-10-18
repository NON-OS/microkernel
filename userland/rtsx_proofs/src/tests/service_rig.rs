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

//! The block service from the outside: requests laid out as a client lays
//! them out, answered by the shipping `serve`, and the one reply it sent
//! read back as a client reads it.

use nonos_libc::{take_sent, To};

use crate::protocol::header::{MAGIC, VERSION};
use crate::protocol::{HDR_LEN, STATUS_LEN};
use crate::serve::medium::Medium;
use crate::serve::request::serve;
use crate::setup::Driver;

pub const KERNEL: u32 = 0;
pub const INSTALLER: u32 = 77;
/// StoreWrite, as abi/caps.toml numbers it.
pub const STORE_WRITE: u64 = 1 << 26;

pub fn request(op: u16, id: u32, body: &[u8]) -> Vec<u8> {
    let (magic, version, len) = (MAGIC.to_le_bytes(), VERSION.to_le_bytes(), body.len() as u32);
    let head =
        [&magic[..], &version, &op.to_le_bytes(), &[0; 4], &id.to_le_bytes(), &len.to_le_bytes()];
    [head.concat().as_slice(), body].concat()
}

pub struct Reply {
    pub to: To,
    pub op: u16,
    pub id: u32,
    pub status: i32,
    pub payload: Vec<u8>,
}

/// What `serve` sent for `rx` from `sender`: exactly one reply, of the
/// length it said, whose header holds its payload length.
pub fn answer_raw(drv: &mut Driver, medium: &mut Medium, sender: u32, rx: &[u8]) -> Reply {
    let mut tx = vec![0u8; HDR_LEN + STATUS_LEN + 128 * 512];
    let used = serve(drv, medium, sender, rx, &mut tx);
    let mut sent = take_sent();
    assert_eq!(sent.len(), 1, "one reply per request");
    let (to, b) = sent.remove(0);
    assert_eq!(b.len(), used);
    let le32 = |at: usize| u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]);
    assert_eq!((le32(0), u16::from_le_bytes([b[4], b[5]])), (MAGIC, VERSION));
    assert_eq!(le32(16) as usize, b.len() - HDR_LEN);
    let (op, id, status) = (u16::from_le_bytes([b[6], b[7]]), le32(12), le32(20) as i32);
    Reply { to, op, id, status, payload: b[HDR_LEN + STATUS_LEN..].to_vec() }
}

pub fn call(drv: &mut Driver, medium: &mut Medium, sender: u32, op: u16, body: &[u8]) -> Reply {
    let r = answer_raw(drv, medium, sender, &request(op, 0x5EED, body));
    assert_eq!((r.op, r.id), (op, 0x5EED), "the reply echoes the request");
    r
}

/// The twelve-byte read or write header.
pub fn span(lba: u64, count: u32) -> Vec<u8> {
    [lba.to_le_bytes().as_slice(), &count.to_le_bytes()].concat()
}
