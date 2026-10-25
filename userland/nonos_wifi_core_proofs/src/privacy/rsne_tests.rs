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

//! Row 1.5: a PSK move's Reassociation Request carries the station's RSNE
//! with no PMKID, byte for byte, and the PMK leaves the station in nothing
//! it sends or replies.

use super::leak::leaked;
use crate::roam::two_aps::{run_until, sent, two_aps, BB};
use crate::soft::join::{PASS, SSID};
use nonos_wifi_core::wpa::psk::psk;

/// CCMP-128 group and pairwise, PSK, capabilities 0, and no PMKID count.
const OURS: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 2, 0, 0];

fn rsne_in(req: &[u8]) -> Option<&[u8]> {
    let mut at = 24 + 10;
    while let (Some(&id), Some(&len)) = (req.get(at), req.get(at + 1)) {
        if id == 48 {
            return req.get(at..at + 2 + usize::from(len));
        }
        at += 2 + usize::from(len);
    }
    None
}

#[test]
fn a_psk_reassociation_names_no_pmkid() {
    let mut st = two_aps(|_| {}, [-85, -50]);
    assert!(run_until(&mut st, |s| s.session().is_some_and(|s| s.bssid == BB)));
    let req = &sent(&st, 0x20)[0];
    assert_eq!(rsne_in(req), Some(&OURS[..]));
    assert!(!leaked(&mut st, &psk(PASS, SSID).unwrap()), "the PMK stays in the station");
}
