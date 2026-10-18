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

//! NBTC requests built and replies read back, as Settings and the shell
//! would, over a board with an in-memory bond store.

use crate::nbtc::who::Caller;
use crate::nbtc::wire::{HDR_LEN, MAGIC, REPLY_MAX, VERSION};
use crate::nbtc::{answer, Board};
use nonos_bt_bond::{BondRecord, MemoryKeys, Store};

pub type TestBoard = Board<MemoryKeys>;

pub const ID: u32 = 0x5eed_0042;
pub const LTK: [u8; 16] = [0xa5; 16];
pub const KB: (u8, [u8; 6]) = (1, [0x11, 0x22, 0x33, 0x44, 0x55, 0xc6]);
pub const MOUSE: (u8, [u8; 6]) = (0, [0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb]);

pub fn board() -> TestBoard {
    let mut b = Board::new(Store::open(MemoryKeys::default()).unwrap());
    b.controller = true;
    b
}

pub fn bonded(b: &mut TestBoard, who: (u8, [u8; 6]), authenticated: bool) {
    let (identity_type, identity) = who;
    let r =
        BondRecord { authenticated, identity_type, identity, ltk: LTK, irk: None, key_size: 16 };
    b.bonds.keep(r).unwrap();
}

pub fn request(op: u16, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0; 4]);
    f.extend_from_slice(&ID.to_le_bytes());
    f.extend_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}

/// The reply's status and payload, the header checked against the request.
pub fn ask(b: &mut TestBoard, who: Caller, op: u16, body: &[u8], now: u64) -> (i32, Vec<u8>) {
    raw(b, who, &request(op, body), now, op)
}

pub fn raw(b: &mut TestBoard, who: Caller, frame: &[u8], now: u64, op: u16) -> (i32, Vec<u8>) {
    let mut tx = [0u8; REPLY_MAX];
    let n = answer(b, who, frame, now, &mut tx);
    reply(&tx[..n], op)
}

pub fn reply(r: &[u8], op: u16) -> (i32, Vec<u8>) {
    let word = |at: usize| u32::from_le_bytes(r[at..at + 4].try_into().unwrap());
    assert_eq!(word(0), MAGIC);
    assert_eq!(r[4..10], [1, 0, op as u8, (op >> 8) as u8, 0, 0]);
    assert_eq!(word(12), ID, "the reply names the request");
    assert_eq!(word(16) as usize, r.len() - HDR_LEN, "the length counts status and payload");
    (word(20) as i32, r[HDR_LEN + 4..].to_vec())
}
