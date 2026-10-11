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

//! Received frames through the station, to net_core.

use nonos_wifi_core::station::{Rx, RxDrop};

use super::buffers::RxBuffers;
use super::state::{RtlLink, RX_DRAIN_MAX};
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::rx::ring::RX_BUF_STRIDE;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// The next Ethernet frame for net_core, if the ring holds one.
    pub(super) fn receive_into(&mut self, out: &mut [u8]) -> Option<usize> {
        // Until the station is associated there is no data traffic to hand up, and
        // draining the ring here would consume the beacons the background scanner
        // needs. Leave the ring untouched so the scanner sees every beacon; net_core
        // has nothing to receive on a link that is not up yet.
        if !self.station.is_associated() {
            return None;
        }
        // Drain past beacons toward a data frame: the air carries far more of them
        // than data and the ring holds only 32 slots, so returning on the first
        // non-data frame let the card overwrite an unread DHCP reply. Bounded to
        // twice the ring so a continuous beacon stream cannot spin this call and
        // starve the caller; the next poll resumes the drain.
        let mut mpdu = [0u8; RX_BUF_STRIDE];
        for _ in 0..RX_DRAIN_MAX {
            let (n, hw_decrypted) = self.poll_frame(&mut mpdu)?;
            self.stats.rx_ring = self.stats.rx_ring.wrapping_add(1);
            let frame = &mpdu[..n];
            if self.note_leave(frame) {
                return None;
            }
            match self.station.receive(frame, hw_decrypted) {
                // A frame larger than the caller's buffer is dropped, never cut.
                Rx::Ethernet(eth) if eth.len() <= out.len() => {
                    self.stats.rx_eth = self.stats.rx_eth.wrapping_add(1);
                    out[..eth.len()].copy_from_slice(&eth);
                    return Some(eth.len());
                }
                Rx::Handshake { frame, group_key } => self.handshake(frame, group_key),
                // Non-data frames (beacons) are the common case and are not news.
                Rx::Dropped(RxDrop::Malformed) => {}
                _ => self.stats.rx_refused = self.stats.rx_refused.wrapping_add(1),
            }
        }
        None
    }
}
