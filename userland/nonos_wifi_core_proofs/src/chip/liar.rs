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
//! A chip that lies at the receive boundary (`Radio::recv`). Between the
//! air's honest frames it gives junk: empty, any length, a length past the
//! caller's buffer, claimed decrypted, at either extreme of signal. A
//! flooding one gives nothing but junk, and never says there is no more.

use crate::soft::rig::SimAir;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::rsn::Ciphers;
use nonos_wifi_core::softmac::{Association, Radio, RxInfo};

pub struct Liar {
    pub air: SimAir,
    pub seed: u64,
    pub flood: bool,
    /// Every `recv` the core made.
    pub calls: u64,
}

impl Radio for Liar {
    fn mac(&self) -> [u8; 6] {
        self.air.mac()
    }
    fn set_mac(&mut self, mac: [u8; 6]) -> bool {
        self.air.set_mac(mac)
    }
    fn bands(&self) -> (bool, bool) {
        self.air.bands()
    }
    fn tune(&mut self, band: Band, channel: u8) -> bool {
        self.air.tune(band, channel)
    }
    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.air.send(mpdu)
    }
    fn recv(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        self.calls += 1;
        if self.flood || self.draw().is_multiple_of(3) {
            return Some(self.junk(out));
        }
        self.air.recv(out)
    }
    fn set_bss(&mut self, bssid: Option<[u8; 6]>) {
        self.air.set_bss(bssid)
    }
    fn associated(&mut self, a: Option<&Association>) {
        self.air.associated(a)
    }
    fn ciphers(&self) -> Ciphers {
        self.air.ciphers()
    }
}
