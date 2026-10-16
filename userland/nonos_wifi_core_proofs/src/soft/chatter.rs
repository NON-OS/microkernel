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

//! The link's own traffic, for the proofs that judge an AP by it: once
//! keyed, an AP that chatters sends the station a protected data frame
//! with each beacon, under the pairwise key and a rising packet number.

use crate::soft::ap::SimAp;
use crate::soft::data_frames::from_ds_of;
use nonos_wifi_core::dot11::{ccmp, gcmp};
use nonos_wifi_core::rsn::TemporalKey;

/// An LLC/SNAP header for IPv4, and a few octets after it.
const LLC: [u8; 12] = [0xaa, 0xaa, 3, 0, 0, 0, 0x08, 0x00, 0x45, 0, 0, 20];

impl SimAp {
    pub fn chat(&mut self, sta: [u8; 6]) -> Option<Vec<u8>> {
        let tk = self.tk.as_ref().filter(|_| self.chatter)?;
        self.pn += 1;
        let plain = from_ds_of(self.bssid, sta, &LLC);
        match tk {
            TemporalKey::Ccmp128(k) => ccmp::encrypt(&plain, self.pn, k),
            TemporalKey::Gcmp256(k) => gcmp::encrypt(&plain, self.pn, k),
        }
    }
}
