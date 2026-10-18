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

//! A chip that restarts. At the radio call it is told, it loses all it was
//! given (its address back to the one it was made with, its channel, its
//! receive filter, every frame waiting), then lies dead for a while,
//! refusing to send or tune and hearing nothing, before it answers again.
//! The air keeps every frame it did send.

use crate::soft::rig::SimAir;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::{Radio, RxInfo};

const DEAD: u64 = 40;

pub struct Restarts {
    pub air: SimAir,
    pub at: u64,
    pub calls: u64,
    pub made: [u8; 6],
}

impl Restarts {
    /// Whether this call finds the chip dead, restarting it on the first.
    fn dead(&mut self) -> bool {
        self.calls += 1;
        if self.calls == self.at {
            (self.air.tuned, self.air.bss, self.air.addr.0) = (None, None, self.made);
            self.air.inbox.clear();
        }
        (self.at..self.at + DEAD).contains(&self.calls)
    }
}

impl Radio for Restarts {
    fn mac(&self) -> [u8; 6] {
        self.air.mac()
    }
    fn set_mac(&mut self, mac: [u8; 6]) -> bool {
        !self.dead() && self.air.set_mac(mac)
    }
    fn bands(&self) -> (bool, bool) {
        self.air.bands()
    }
    fn tune(&mut self, band: Band, channel: u8) -> bool {
        !self.dead() && self.air.tune(band, channel)
    }
    fn send(&mut self, mpdu: &[u8]) -> bool {
        !self.dead() && self.air.send(mpdu)
    }
    fn recv(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        if self.dead() {
            return None;
        }
        self.air.recv(out)
    }
    fn set_bss(&mut self, bssid: Option<[u8; 6]>) {
        if !self.dead() {
            self.air.set_bss(bssid)
        }
    }
}
