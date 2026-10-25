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

//! A frame queued for the device, and the doorbell rung.

use nonos_wifi_core::dot11::ccmp::view;

use super::super::cmdq::FIRST_TB;
use super::super::region::Region;
use super::super::regs::HBUS_TARG_WRPTR;
use super::super::tx_cmd::{header, TX_CMD, TX_CMD_GEN3_HDR};
use super::layout::{CMD_HDR, TXQ_ENTRIES, TXQ_FRAMES, TX_BUF};
use super::queue::TxQueue;
use crate::regs::Mmio;

impl TxQueue {
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

        let Some(head) = cmd.get(..FIRST_TB + tb1) else { return false };
        if !self.write_slot(tx, slot, head, body, tb1, frame.len()) {
            return false;
        }
        self.write = self.write.wrapping_add(1);
        m.write32(HBUS_TARG_WRPTR, u32::from(self.write) | u32::from(self.id) << 16);
        true
    }
}
