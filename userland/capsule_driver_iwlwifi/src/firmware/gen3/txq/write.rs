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

//! A frame's buffers, and its slot's descriptor and byte count entry.

use super::super::cmdq::FIRST_TB;
use super::super::plan::{FIRST_TB_SLOT, TFD_SIZE};
use super::super::region::{put16, put64, zero, Region};
use super::layout::{BC, BUFS, FIRST, TFDS, TXQ_FRAMES, TX_BUF};
use super::queue::TxQueue;

impl TxQueue {
    // Write the command head (`head`, its first buffer and the `tb1` bytes
    // after it), the body, and the descriptor and byte count entry of `slot`
    // for a frame of `len` bytes.
    pub(super) fn write_slot<R: Region + ?Sized>(
        &self,
        tx: &R,
        slot: usize,
        head: &[u8],
        body: &[u8],
        tb1: usize,
        len: usize,
    ) -> bool {
        let buf = self.base + BUFS + (self.write as usize % TXQ_FRAMES) * TX_BUF;
        let first = self.base + FIRST + slot * FIRST_TB_SLOT;
        let tfd = self.base + TFDS + slot * TFD_SIZE;
        let at = |off: usize| tx.dev() + off as u64;
        let buffers: u16 = if body.is_empty() { 2 } else { 3 };
        let chunks = (2 + 10 * buffers).div_ceil(64) - 1;
        let bc = len as u16 | chunks << 14;
        // The buffer cleared first: no byte of an earlier frame stays.
        zero(tx, buf, TX_BUF)
            && tx.write(buf, head)
            && tx.write(buf + FIRST_TB + tb1, body)
            && tx.write(first, &head[..FIRST_TB])
            && zero(tx, tfd, TFD_SIZE)
            && set_tb(tx, tfd, 0, FIRST_TB, at(first))
            && set_tb(tx, tfd, 1, tb1, at(buf + FIRST_TB))
            && (body.is_empty() || set_tb(tx, tfd, 2, body.len(), at(buf + FIRST_TB + tb1)))
            && put16(tx, tfd, buffers)
            && put16(tx, self.base + BC + slot * 2, bc)
    }
}

// Buffer `i` of the descriptor at `tfd`: `[len u16][addr u64]` after the count.
fn set_tb<R: Region + ?Sized>(tx: &R, tfd: usize, i: usize, len: usize, addr: u64) -> bool {
    let at = tfd + 2 + i * 10;
    put16(tx, at, len as u16) && put64(tx, at + 2, addr)
}
