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
//! One received frame through the station.
use alloc::vec::Vec;
use nonos_wifi_core::station::{Loss, Rx, RxDrop};

use super::super::super::rx_data::RxMpdu;
use super::state::{Link, MIC_LEN, RX_READY};

impl Link {
    /// Take one received frame; the frame to send back, if any. A frame that
    /// ends the link leaves why in `left`.
    pub fn receive(&mut self, rx: RxMpdu) -> Option<Vec<u8>> {
        if !self.station.is_associated() {
            return None;
        }
        self.stats.rx_frames = self.stats.rx_frames.wrapping_add(1);
        let mut frame = rx.frame;
        if rx.decrypted {
            frame.extend_from_slice(&[0u8; MIC_LEN]);
        }
        match self.station.receive(&frame, rx.decrypted) {
            Rx::Ethernet(eth) => {
                if self.ready.len() >= RX_READY {
                    self.stats.rx_full = self.stats.rx_full.wrapping_add(1);
                } else {
                    self.stats.rx_eth = self.stats.rx_eth.wrapping_add(1);
                    self.ready.push_back(eth);
                }
                None
            }
            Rx::Handshake { frame, group_key, pairwise_key } => {
                self.hold_keys(group_key, pairwise_key);
                frame
            }
            Rx::Send(frame) => Some(frame),
            Rx::Lost(why) => {
                self.lost(why);
                None
            }
            // Management frames (beacons above all) are the common case.
            Rx::Dropped(RxDrop::Malformed | RxDrop::Management) => None,
            Rx::Dropped(_) => {
                self.stats.rx_refused = self.stats.rx_refused.wrapping_add(1);
                None
            }
        }
    }

    /// Note why the link ended, for the serving loop to close it.
    pub fn lost(&mut self, why: Loss) {
        match why {
            Loss::Left(reason) | Loss::Unanswered(reason) => self.left = Some(reason),
            Loss::Silent => self.silent = true,
        }
    }
}
