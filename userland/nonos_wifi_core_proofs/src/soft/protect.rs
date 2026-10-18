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

//! A simulated AP's beacon protection: once keyed, each beacon it sends ends
//! with an MME under the BIGTK it gave in message 3, its BIPN one higher
//! than the last.

use crate::soft::ap::SimAp;
use nonos_wifi_core::pmf::bip_protect_beacon;

pub const BIGTK: [u8; 16] = [0x66; 16];

impl SimAp {
    /// The next beacon on the air: protected once the AP is keyed and runs
    /// beacon protection, bare otherwise.
    pub fn beacon_out(&mut self) -> Vec<u8> {
        let b = self.beacon();
        let q = &self.quirks.beacons;
        if !q.protect || q.bare || self.tk.is_none() {
            return b;
        }
        self.quirks.beacons.bipn += 1;
        bip_protect_beacon(&b, 6, self.quirks.beacons.bipn, &BIGTK).expect("a beacon")
    }
}
