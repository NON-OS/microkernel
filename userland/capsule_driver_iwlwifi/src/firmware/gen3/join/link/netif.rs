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
//! The `LinkPort` net_core speaks to, over the open port.
use super::super::super::region::{Clock, Region};
use super::super::fw::Queue;
use super::port::Port;
use crate::regs::Mmio;
use nonos_wifi_core::netif::LinkPort;
impl<M: Mmio, R: Region + ?Sized, C: Clock> LinkPort for Port<'_, '_, '_, M, R, C> {
    fn mac(&self) -> Option<[u8; 6]> {
        self.link.station.is_associated().then(|| self.link.station.mac())
    }

    fn link_up(&self) -> bool {
        self.link.station.is_associated()
    }

    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize> {
        self.service();
        while let Some(eth) = self.link.ready.pop_front() {
            // A frame larger than the caller's buffer is dropped, never cut.
            if let Some(dst) = out.get_mut(..eth.len()) {
                dst.copy_from_slice(&eth);
                return Some(eth.len());
            }
            self.link.stats.rx_full = self.link.stats.rx_full.wrapping_add(1);
        }
        None
    }
    fn send_tx(&mut self, frame: &[u8]) -> bool {
        let sent = match self.link.station.tx_frame(frame) {
            Some(mpdu) => self.fw.send(Queue::Data, &mpdu, self.link.data_rate),
            None => false,
        };
        let s = &mut self.link.stats;
        if sent {
            s.tx_ok = s.tx_ok.wrapping_add(1);
        } else {
            s.tx_drop = s.tx_drop.wrapping_add(1);
        }
        sent
    }
}
