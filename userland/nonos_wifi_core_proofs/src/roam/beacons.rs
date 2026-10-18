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

//! Beacons for the roaming proofs: any AP, any network name, any RSNE,
//! laid out as IEEE Std 802.11-2020, 9.3.3.2 has a beacon.

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::roam::Heard;

pub const RSNE_PSK: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 2, 0, 0];
/// SAE only.
pub const RSNE_SAE: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 8, 0x80, 0];
/// PSK, with management frame protection required.
pub const RSNE_PSK_MFPR: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 2, 0xc0, 0];

pub fn ap(n: u8) -> [u8; 6] {
    [0x02, 0, 0, 0, 0x0a, n]
}

pub fn beacon_of(bssid: [u8; 6], ssid: &[u8], channel: u8, rsne: &[u8]) -> Vec<u8> {
    let mut f = vec![0x80, 0, 0, 0];
    f.extend_from_slice(&[0xff; 6]);
    f.extend_from_slice(&bssid);
    f.extend_from_slice(&bssid);
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&[0u8; 8]);
    f.extend_from_slice(&[0x64, 0, 0x11, 0x04, 0, ssid.len() as u8]);
    f.extend_from_slice(ssid);
    f.extend_from_slice(&[1, 8, 0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24]);
    f.extend_from_slice(&[3, 1, channel]);
    f.extend_from_slice(rsne);
    f
}

pub fn heard(n: u8, ssid: &[u8], rssi_dbm: i8, rsne: &[u8]) -> Heard {
    let channel = 1 + n % 11;
    let beacon = beacon_of(ap(n), ssid, channel, rsne);
    Heard { bssid: ap(n), band: Band::Ghz2, channel, rssi_dbm, beacon }
}
