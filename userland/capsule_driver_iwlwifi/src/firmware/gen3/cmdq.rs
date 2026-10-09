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

//! The host command queue of an AX210-family part (Linux v6.12 pcie/tx-gen2.c
//! `iwl_pcie_gen2_enqueue_hcmd`).
//!
//! A command is the 8-byte wide header (`iwl_cmd_header_wide`: opcode, group,
//! sequence, payload length, version) and its payload. It is handed to the
//! device by a 256-byte TFH descriptor (`iwl_tfh_tfd`: a buffer count and up
//! to 25 `[len u16][addr u64]` buffers): the first buffer is the command's
//! first 20 bytes copied into the slot's own small buffer, the second the
//! rest. The write pointer runs over 16 bits (`max_tfd_queue_size` 65536),
//! the descriptor slot is it modulo the 128-entry ring, and the doorbell is
//! `HBUS_TARG_WRPTR = write_ptr | queue << 16`. The sequence (`queue << 8 |
//! index`) is what the firmware's reply carries back. Queue 0 is the command
//! queue (`IWL_MVM_DQA_CMD_QUEUE`).

use super::plan::{CMD_BUF, CMD_BUFFER, CMD_RING, CMD_TFDS, FIRST_TBS, FIRST_TB_SLOT, TFD_SIZE};
use super::region::{put16, put64, zero, Region};
use super::regs::HBUS_TARG_WRPTR;
use crate::regs::Mmio;

/// `IWL_FIRST_TB_SIZE`.
pub const FIRST_TB: usize = 20;
pub const WIDE_HDR: usize = 8;
const QUEUE: u16 = 0;

pub struct CmdQueue {
    write_ptr: u16,
}

impl Default for CmdQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl CmdQueue {
    pub const fn new() -> Self {
        Self { write_ptr: 0 }
    }

    /// Clear the descriptor ring and the first-buffer slots.
    pub fn init<R: Region + ?Sized>(&mut self, ctrl: &R) -> bool {
        self.write_ptr = 0;
        zero(ctrl, CMD_TFDS, CMD_RING * TFD_SIZE) && zero(ctrl, FIRST_TBS, CMD_RING * FIRST_TB_SLOT)
    }

    /// Queue command `cmd` of `group` (version `version`, usually 0) with
    /// `payload`, ring the doorbell, and return the sequence its reply will
    /// carry. `None` if the command does not fit the command buffer.
    pub fn send<M: Mmio, R: Region + ?Sized>(
        &mut self,
        m: &M,
        ctrl: &R,
        group: u8,
        cmd: u8,
        version: u8,
        payload: &[u8],
    ) -> Option<u16> {
        let total = WIDE_HDR.checked_add(payload.len())?;
        if total > CMD_BUF || payload.len() > u16::MAX as usize {
            return None;
        }
        let slot = self.write_ptr as usize & (CMD_RING - 1);
        let sequence = (QUEUE & 0x1F) << 8 | (self.write_ptr & 0xFF);
        let mut hdr = [0u8; WIDE_HDR];
        hdr[0] = cmd;
        hdr[1] = group;
        hdr[2..4].copy_from_slice(&sequence.to_le_bytes());
        hdr[4..6].copy_from_slice(&(payload.len() as u16).to_le_bytes());
        hdr[7] = version;
        if !ctrl.write(CMD_BUFFER, &hdr) || !ctrl.write(CMD_BUFFER + WIDE_HDR, payload) {
            return None;
        }
        // The first 20 bytes (or the whole command, if shorter) in the slot's
        // own first-buffer slot.
        let tb0 = total.min(FIRST_TB);
        let mut first = [0u8; FIRST_TB];
        first[..WIDE_HDR].copy_from_slice(&hdr);
        let from_payload = tb0 - WIDE_HDR;
        first[WIDE_HDR..tb0].copy_from_slice(&payload[..from_payload]);
        let first_at = FIRST_TBS + slot * FIRST_TB_SLOT;
        if !ctrl.write(first_at, &first[..tb0]) {
            return None;
        }
        let tfd = CMD_TFDS + slot * TFD_SIZE;
        if !zero(ctrl, tfd, TFD_SIZE) || !set_tb(ctrl, tfd, 0, tb0, ctrl.dev() + first_at as u64) {
            return None;
        }
        let mut tbs = 1u16;
        if total > tb0 {
            let rest = ctrl.dev() + (CMD_BUFFER + tb0) as u64;
            if !set_tb(ctrl, tfd, 1, total - tb0, rest) {
                return None;
            }
            tbs = 2;
        }
        if !put16(ctrl, tfd, tbs) {
            return None;
        }
        self.write_ptr = self.write_ptr.wrapping_add(1);
        m.write32(HBUS_TARG_WRPTR, self.write_ptr as u32 | (QUEUE as u32) << 16);
        Some(sequence)
    }
}

// Buffer `i` of the descriptor at `tfd`: `[len u16][addr u64]` after the count.
fn set_tb<R: Region + ?Sized>(ctrl: &R, tfd: usize, i: usize, len: usize, addr: u64) -> bool {
    let at = tfd + 2 + i * 10;
    put16(ctrl, at, len as u16) && put64(ctrl, at + 2, addr)
}
