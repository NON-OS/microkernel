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

//! The one frame the beacon hunt may transmit. A network that broadcasts its
//! name is found by listening, so nothing is sent for it and no saved
//! network's name leaves the machine. Only a network the person marked hidden
//! (its beacons carry no name) gets a probe request, naming it alone, sent
//! from the per-boot random address. Kept apart from the hunt's radio loop so
//! the proofs check the decision and the frame on the host.

use nonos_wifi_core::dot11::mgmt::probe_request;

/// The rates a probe request offers: the 802.11b/g set, CCK ones basic.
pub const PROBE_RATES: [u8; 8] = [0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24];

/// The probe the hunt sends on its `step`th channel into `out`: its length,
/// or `None` when the network is not hidden (or `out` cannot hold it).
pub fn hunt_probe(out: &mut [u8], hidden: bool, our_mac: [u8; 6], ssid: &[u8], step: usize) -> Option<usize> {
    if !hidden {
        return None;
    }
    probe_request(out, our_mac, ssid, &PROBE_RATES, u16::try_from(step).ok()?)
}
