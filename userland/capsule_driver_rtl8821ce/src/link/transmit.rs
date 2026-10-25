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

//! One Ethernet frame from net_core onto the transmit ring.

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::tx::desc::{FrameMeta, DESC_RATE_6M};
use crate::tx::enqueue;
use crate::tx::regs::QSEL_BE;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Frame and queue one Ethernet frame from net_core, counting it sent or
    /// dropped.
    pub(super) fn transmit(&mut self, frame: &[u8]) -> bool {
        let Some(mpdu) = self.station.tx_frame(frame) else {
            self.stats.tx_drop = self.stats.tx_drop.wrapping_add(1);
            return false;
        };
        // Keep the descriptor sequence consistent with the header the station
        // wrote (seq control lives at offset 22, the number in its top 12 bits).
        let seq = if mpdu.len() >= 24 {
            (u16::from_le_bytes([mpdu[22], mpdu[23]]) >> 4) & 0x0FFF
        } else {
            0
        };
        // Fix the transmit rate rather than leaving it to firmware rate control.
        // The firmware has no rate-adaptation table for this station (the driver
        // never registers one after association), so a rate-controlled data frame
        // is dropped before it reaches the air; a fixed rate transmits, exactly as
        // the fixed-rate handshake frames already do.
        // No security type: `tx_frame` has already encrypted the frame in
        // software (header, CCMP header, MIC). A descriptor tagged CCMP asks
        // the MAC to encrypt it again, as rtw88 does only for frames with a
        // hardware key, and the AP then fails the MIC on every data frame.
        let meta = FrameMeta { qsel: QSEL_BE, bmc: false, rate: Some(DESC_RATE_6M), seq, sec_type: 0 };
        let ok =
            enqueue(&self.mmio, &self.tx_ring, &self.tx_buffers, &mut self.tx_state, &mpdu, &meta);
        if ok {
            self.stats.tx_ok = self.stats.tx_ok.wrapping_add(1);
        } else {
            self.stats.tx_drop = self.stats.tx_drop.wrapping_add(1);
        }
        ok
    }
}
