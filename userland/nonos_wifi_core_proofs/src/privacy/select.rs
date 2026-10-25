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

//! RSNEs differing in one field each, for the proofs that a roam keeps the
//! whole security its session runs (IEEE Std 802.11-2020, 9.4.2.24).

use crate::roam::beacons::{ap, beacon_of};
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::rsn::{Ciphers, Selection};
use nonos_wifi_core::softmac::roam::{group_mgmt_of, offered, pick, roam_policy, Heard, Want};

pub const HOME: &[u8] = b"home";
pub const CCMP: u8 = 4;
pub const GCMP256: u8 = 9;
pub const BIP_CMAC: u8 = 6;
pub const BIP_GMAC256: u8 = 12;
/// The RSNXE with Hash-to-Element (bit 5).
pub const RSNXE_H2E: [u8; 3] = [0xf4, 1, 0x20];

/// One group, one pairwise and one AKM suite, the capabilities, and a
/// group management suite when given.
pub fn rsne(group: u8, pairwise: u8, akm: u8, caps: u8, mgmt: Option<u8>) -> Vec<u8> {
    let s = |t: u8| [0, 0x0f, 0xac, t];
    let mut b = vec![1, 0];
    b.extend(s(group));
    b.extend([1, 0]);
    b.extend(s(pairwise));
    b.extend([1, 0]);
    b.extend(s(akm));
    b.extend([caps, 0]);
    if let Some(m) = mgmt {
        b.extend([0, 0]);
        b.extend(s(m));
    }
    [vec![0x30, b.len() as u8], b].concat()
}

const RADIO: Ciphers = Ciphers { gcmp256: true };

/// What the session on `elements` runs, and its AP's group management.
pub fn session(elements: &[u8]) -> (Selection, Option<u32>) {
    let b = beacon_of(ap(1), HOME, 1, elements);
    let any = nonos_wifi_core::rsn::JoinPolicy::ANY.with_ciphers(RADIO);
    let runs = offered(&b, any).expect("a session");
    (runs, group_mgmt_of(&b))
}

/// Whether a candidate with `elements`, heard far stronger, is picked by
/// the session on `ours`.
pub fn picked(ours: &[u8], elements: &[u8]) -> bool {
    let (runs, group_mgmt) = session(ours);
    let policy = roam_policy(&runs, RADIO);
    let want = Want { ssid: HOME, runs: &runs, group_mgmt, policy, barred: &[] };
    let beacon = beacon_of(ap(2), HOME, 6, elements);
    let h = Heard { bssid: ap(2), band: Band::Ghz2, channel: 6, rssi_dbm: -30, beacon };
    pick((ap(1), -90), &want, &[h]) == Some(0)
}
