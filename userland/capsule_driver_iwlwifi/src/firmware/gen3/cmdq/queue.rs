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

//! The command queue's write pointer, and a command sent on it.

use super::super::plan::{CMD_BUF, CMD_RING, CMD_TFDS, FIRST_TBS, FIRST_TB_SLOT, TFD_SIZE};
use super::super::region::{zero, Region};
use super::super::regs::HBUS_TARG_WRPTR;
use super::sizes::{QUEUE, WIDE_HDR};
use super::write::{header, write_command};
use crate::regs::Mmio;

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
        let hdr = header(group, cmd, version, sequence, payload.len() as u16);
        if !write_command(ctrl, slot, &hdr, payload) {
            return None;
        }
        self.write_ptr = self.write_ptr.wrapping_add(1);
        m.write32(HBUS_TARG_WRPTR, self.write_ptr as u32 | (QUEUE as u32) << 16);
        Some(sequence)
    }
}
