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

//! What the driver tells the station's clock besides the time: the radio's
//! notice that the access point's beacons stopped, and the SSID the probes
//! after it name.

use super::watch::SSID_MAX;
use super::LinkStation;

impl LinkStation {
    /// The radio reports the access point's beacons missed: ask after it at
    /// the next tick rather than wait out the quiet.
    pub fn beacons_missed(&mut self) {
        if let Some(now) = self.watch.clock {
            self.watch.alive.missed(now);
        }
    }

    /// The network's SSID, for the probes; given after `associate`, which
    /// clears it. One longer than an SSID is ignored.
    pub fn set_ssid(&mut self, ssid: &[u8]) {
        if ssid.len() <= SSID_MAX {
            self.watch.ssid[..ssid.len()].copy_from_slice(ssid);
            self.watch.ssid_len = ssid.len();
        }
    }
}
