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

//! The frames the simulated access point sends: beacons and probe responses
//! naming its network, its authentication and association answers, and a
//! deauthentication; EAPOL in data frames is `data_frames`.

pub use super::data_frames::{eapol_from, eapol_of, from_ds};
use crate::ap_sim::AA;

pub const RSNE_PSK: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 2, 0, 0];
const RATES: [u8; 10] = [1, 8, 0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24];

pub fn mgmt(subtype: u8, dst: [u8; 6], body: &[u8]) -> Vec<u8> {
    mgmt_from(AA, subtype, dst, body)
}

/// A management frame from the AP `from` (its BSSID too).
pub fn mgmt_from(from: [u8; 6], subtype: u8, dst: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![subtype << 4, 0, 0, 0];
    f.extend_from_slice(&dst);
    f.extend_from_slice(&from);
    f.extend_from_slice(&from);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(body);
    f
}

/// A beacon (`to` broadcast) or probe response from the AP `from`.
pub fn beacon_from(from: [u8; 6], subtype: u8, to: [u8; 6], net: (&[u8], u8, &[u8])) -> Vec<u8> {
    let (ssid, channel, rsne) = net;
    let mut body = vec![0u8; 8];
    body.extend_from_slice(&[0x64, 0, 0x11, 0x04, 0, ssid.len() as u8]);
    body.extend_from_slice(ssid);
    body.extend_from_slice(&RATES);
    body.extend_from_slice(&[3, 1, channel]);
    body.extend_from_slice(rsne);
    mgmt_from(from, subtype, to, &body)
}
