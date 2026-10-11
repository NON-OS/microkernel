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

//! Frames onto the transmit ring, and the handshakes the link answers.
use alloc::vec::Vec;

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::tx::desc::{FrameMeta, DESC_RATE_6M};
use crate::tx::enqueue;
use crate::tx::regs::QSEL_BE;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    // Send what a key handshake on the link produced, and keep its new key.
    pub(super) fn handshake(&mut self, frame: Option<Vec<u8>>, group_key: Option<(u8, [u8; 16])>) {
        if let Some(f) = frame {
            self.send_raw(&f);
        }
        if group_key.is_some() {
            self.new_group_key = group_key;
            self.stats.rekeys = self.stats.rekeys.wrapping_add(1);
        }
    }

    /// Transmit a fully-formed 802.11 frame unencrypted, at the robust early
    /// association rate. The authentication, association and four-way-handshake
    /// frames all go out this way, before any key exists, so the security type is
    /// none. The sequence number is taken from the frame's own header.
    pub fn send_raw(&mut self, mpdu: &[u8]) -> bool {
        let seq = if mpdu.len() >= 24 {
            (u16::from_le_bytes([mpdu[22], mpdu[23]]) >> 4) & 0x0FFF
        } else {
            0
        };
        let meta =
            FrameMeta { qsel: QSEL_BE, bmc: false, rate: Some(DESC_RATE_6M), seq, sec_type: 0 };
        enqueue(&self.mmio, &self.tx_ring, &self.tx_buffers, &mut self.tx_state, mpdu, &meta)
    }
}
