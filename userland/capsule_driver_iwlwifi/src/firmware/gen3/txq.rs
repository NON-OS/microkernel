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

//! One transmit queue of an AX210-family part (Linux v6.12 pcie/tx-gen2.c
//! `iwl_txq_gen2_tx`, `iwl_txq_gen2_build_tx`, `iwl_pcie_gen2_update_byte_tbl`,
//! `iwl_txq_reclaim`).
//!
//! The firmware owns the queue's number and scheduling (TVQM); the host owns
//! its memory: a ring of 256-byte TFH descriptors, a byte count table of
//! 1024 `iwl_gen3_bc_tbl_entry` words (`TFD_QUEUE_BC_SIZE_GEN3_AX210`), a
//! 64-byte first-buffer slot per descriptor, and the frames themselves. A
//! frame goes out as the device command `[iwl_cmd_header][iwl_tx_cmd_gen3]
//! [802.11 header]` then its body: the first buffer is that command's first
//! 20 bytes (`IWL_FIRST_TB_SIZE`) copied into the slot, the second the rest of
//! the command and header rounded up to four bytes, the third the body. The
//! command header is `TX_CMD`, group 0, and the sequence `(queue & 0x1F) << 8
//! | index`. The byte count entry is the frame's length with the descriptor's
//! fetch chunks (`DIV_ROUND_UP(2 + 10 * buffers, 64) - 1`) in bits 14-15, and
//! the doorbell is `HBUS_TARG_WRPTR = write_ptr | queue << 16`, the write
//! pointer running over 16 bits (`max_tfd_queue_size` 65536) and the
//! descriptor index being it modulo the ring.
//!
//! The firmware answers each frame with a TX response naming the queue and
//! the next index it has not finished (`ssn`); the slots before it are free
//! again. A response naming more slots than are in flight is ignored (Linux
//! logs it and reclaims nothing). At most [`TXQ_FRAMES`] frames are in flight,
//! each in its own buffer, so a buffer is never rewritten while the device
//! may still read it.

use nonos_wifi_core::dot11::ccmp::view;

use super::cmdq::FIRST_TB;
use super::plan::{FIRST_TB_SLOT, TFD_SIZE};
use super::region::{put16, put64, zero, Region};
use super::regs::HBUS_TARG_WRPTR;
use super::tx_cmd::{header, TX_CMD, TX_CMD_GEN3_HDR};
use crate::regs::Mmio;

/// Descriptors per queue: `max(IWL_MGMT_QUEUE_SIZE, min_txq_size)` for
/// AX210, and the size every TVQM queue here is asked for.
pub const TXQ_ENTRIES: usize = 128;
/// Frames in flight per queue.
pub const TXQ_FRAMES: usize = 16;
/// One frame's buffer: the command, header and body of the largest frame.
pub const TX_BUF: usize = 2048;
/// `TFD_QUEUE_BC_SIZE_GEN3_AX210` entries of two bytes.
pub const BC_TABLE: usize = 1024 * 2;
/// The device command header before the transmit command.
const CMD_HDR: usize = 4;

// Offsets inside one queue's area.
const TFDS: usize = 0;
const BC: usize = TFDS + TXQ_ENTRIES * TFD_SIZE;
const FIRST: usize = BC + ((BC_TABLE + 4095) & !4095);
const BUFS: usize = FIRST + TXQ_ENTRIES * FIRST_TB_SLOT;
/// One queue's area, rounded to pages.
pub const TXQ_STRIDE: usize = (BUFS + TXQ_FRAMES * TX_BUF + 4095) & !4095;
/// The transmit region: the management queue's area, then the data queue's.
pub const TX_REGION: usize = 2 * TXQ_STRIDE;

pub struct TxQueue {
    base: usize,
    /// The firmware's number for the queue, while it has one.
    pub id: u16,
    pub live: bool,
    write: u16,
    read: u16,
}

impl TxQueue {
    /// The queue whose area starts `base` bytes into the transmit region.
    pub const fn new(base: usize) -> Self {
        Self { base, id: 0, live: false, write: 0, read: 0 }
    }

    /// The descriptor ring's and byte count table's device addresses.
    pub fn tfds<R: Region + ?Sized>(&self, tx: &R) -> u64 {
        tx.dev() + (self.base + TFDS) as u64
    }

    pub fn bc<R: Region + ?Sized>(&self, tx: &R) -> u64 {
        tx.dev() + (self.base + BC) as u64
    }

    /// Clear the queue's memory before the firmware is told of it.
    pub fn clear<R: Region + ?Sized>(&mut self, tx: &R) -> bool {
        self.live = false;
        self.write = 0;
        self.read = 0;
        zero(tx, self.base, BUFS)
    }

    /// Take the firmware's number for the queue and the write pointer its
    /// ring starts at. Until then (and after `clear`) nothing is sent.
    pub fn start(&mut self, id: u16, write_ptr: u16) {
        self.id = id;
        self.live = true;
        self.write = write_ptr;
        self.read = write_ptr;
    }

    /// Frames the device has not answered.
    pub fn in_flight(&self) -> usize {
        self.write.wrapping_sub(self.read) as usize
    }

    /// Queue `frame` (a whole 802.11 frame, any encryption already applied)
    /// at `rate_n_flags` and ring the doorbell. `false`, with nothing
    /// written to the device, when the queue has no firmware number or is
    /// full, the frame is malformed or too long, or a write falls outside
    /// the region.
    pub fn send<M: Mmio, R: Region + ?Sized>(&mut self, m: &M, tx: &R, frame: &[u8], rate_n_flags: u32) -> bool {
        if !self.live || self.in_flight() >= TXQ_FRAMES {
            return false;
        }
        let Some(hdr_len) = view(frame).map(|v| v.hdr_len) else { return false };
        let Some(txc) = header(frame.len(), hdr_len, rate_n_flags) else { return false };
        // The command and header past the first buffer, rounded to four.
        let cmd_len = CMD_HDR + TX_CMD_GEN3_HDR + hdr_len;
        let tb1 = (cmd_len - FIRST_TB + 3) & !3;
        let body = &frame[hdr_len..];
        if FIRST_TB + tb1 + body.len() > TX_BUF {
            return false;
        }
        let slot = self.write as usize & (TXQ_ENTRIES - 1);
        let sequence = (self.id & 0x1F) << 8 | slot as u16;
        let mut cmd = [0u8; CMD_HDR + TX_CMD_GEN3_HDR + 32 + 4];
        cmd[0] = TX_CMD;
        cmd[2..4].copy_from_slice(&sequence.to_le_bytes());
        cmd[CMD_HDR..CMD_HDR + TX_CMD_GEN3_HDR].copy_from_slice(&txc);
        let Some(h) = cmd.get_mut(CMD_HDR + TX_CMD_GEN3_HDR..cmd_len) else { return false };
        h.copy_from_slice(&frame[..hdr_len]);

        let buf = self.base + BUFS + (self.write as usize % TXQ_FRAMES) * TX_BUF;
        let first = self.base + FIRST + slot * FIRST_TB_SLOT;
        let tfd = self.base + TFDS + slot * TFD_SIZE;
        let at = |off: usize| tx.dev() + off as u64;
        let buffers: u16 = if body.is_empty() { 2 } else { 3 };
        let chunks = (2 + 10 * buffers).div_ceil(64) - 1;
        let bc = frame.len() as u16 | chunks << 14;
        let Some(head) = cmd.get(..FIRST_TB + tb1) else { return false };
        let ok = tx.write(buf, head)
            && tx.write(buf + FIRST_TB + tb1, body)
            && tx.write(first, &cmd[..FIRST_TB])
            && zero(tx, tfd, TFD_SIZE)
            && set_tb(tx, tfd, 0, FIRST_TB, at(first))
            && set_tb(tx, tfd, 1, tb1, at(buf + FIRST_TB))
            && (body.is_empty() || set_tb(tx, tfd, 2, body.len(), at(buf + FIRST_TB + tb1)))
            && put16(tx, tfd, buffers)
            && put16(tx, self.base + BC + slot * 2, bc);
        if !ok {
            return false;
        }
        self.write = self.write.wrapping_add(1);
        m.write32(HBUS_TARG_WRPTR, u32::from(self.write) | u32::from(self.id) << 16);
        true
    }

    /// The device finished every slot before `ssn`: free them. `false` when
    /// that names more slots than are in flight (nothing is freed).
    pub fn reclaim(&mut self, ssn: u16) -> bool {
        let target = ssn as usize & (TXQ_ENTRIES - 1);
        let read = self.read as usize & (TXQ_ENTRIES - 1);
        let n = target.wrapping_sub(read) & (TXQ_ENTRIES - 1);
        if n > self.in_flight() {
            return false;
        }
        self.read = self.read.wrapping_add(n as u16);
        true
    }
}

// Buffer `i` of the descriptor at `tfd`: `[len u16][addr u64]` after the count.
fn set_tb<R: Region + ?Sized>(tx: &R, tfd: usize, i: usize, len: usize, addr: u64) -> bool {
    let at = tfd + 2 + i * 10;
    put16(tx, at, len as u16) && put64(tx, at + 2, addr)
}
