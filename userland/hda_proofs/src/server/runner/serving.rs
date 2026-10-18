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

//! A driver that plays, brought up on a `board`, and the requests the
//! serving loop hands its handlers, with the buffer it replies from.

use nonos_libc::replies;

use super::max_tx_body::max_tx_body;
use crate::constants::SD_LPIB;
use crate::controller::bdl::POSBUF_OFFSET;
use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::proofs::lifecycle::attempt::attempt_until;
use crate::proofs::lifecycle::board::{board, Board, LOW};
use crate::protocol::{Request, RESP_HDR_LEN, STATUS_LEN};
use crate::setup::{Driver, Started};

/// The driver and the board it runs on. Fields drop in order, so the driver
/// stops the controller before the board's memory goes.
pub struct Playing {
    pub d: Driver,
    pub b: Board,
}

/// The HP 15s's ALC236 on QEMU's controller, brought up to play.
pub fn playing() -> Playing {
    let alc236 = || board(vec![(0, sim_codec(&alc236_hp()))], &LOW);
    let (b, r, _) = attempt_until(alc236, |r, _| matches!(r, Ok(Started::Playing(_))));
    let Ok(Started::Playing(d)) = r else { panic!("the ALC236 did not play") };
    Playing { d: *d, b }
}

pub const ID: u32 = 0x5150;

/// A request for `op` carrying `payload_len` bytes, as decoded.
pub fn ask(op: u16, payload_len: u32) -> Request {
    Request { op, flags: 0, request_id: ID, payload_len }
}

/// The buffer the loop replies from, as long as `run` makes it.
pub fn tx() -> Vec<u8> {
    vec![0u8; RESP_HDR_LEN + 4 + max_tx_body()]
}

/// The last reply sent, held to be one whole frame for request `op`: its
/// status word, and the body after it.
pub fn answer(op: u16) -> (i32, Vec<u8>) {
    let r = replies().last().cloned().expect("nothing was sent");
    let le32 = |o: usize| u32::from_le_bytes([r[o], r[o + 1], r[o + 2], r[o + 3]]);
    assert_eq!(u16::from_le_bytes([r[6], r[7]]), op, "the reply names another op");
    assert_eq!(le32(12), ID, "the reply names another request");
    assert_eq!(le32(16) as usize + RESP_HDR_LEN, r.len(), "the length is not the frame's");
    (le32(RESP_HDR_LEN) as i32, r[RESP_HDR_LEN + STATUS_LEN..].to_vec())
}

/// The controller reports `lpib` in LPIB and `buffer` in the stream's
/// position buffer entry.
pub fn at(p: &Playing, lpib: u32, buffer: u32) {
    p.b.sim.bar.present32((p.d.stream_off + SD_LPIB) as usize, lpib);
    p.b.bdl.present32(POSBUF_OFFSET as usize + p.d.stream_gi as usize * 8, buffer);
}
