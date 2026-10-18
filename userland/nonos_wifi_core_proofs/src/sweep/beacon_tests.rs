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
//! Beacons and probe responses whole: the SSID, channel and RSN the
//! building says, before and after a lying length; another frame type or
//! subtype, or a frame without room for its fixed fields, is not one.

use super::frame_tests::mgmt;
use super::model::{build, Built};
use super::rng::{each, within};
use nonos_wifi_core::dot11::parse::parse_beacon;

/// The frame's header and fixed fields, then `b`'s elements; and what
/// the first `honest` of them say.
fn check(f: &[u8], b: &Built, honest: usize) {
    let first = |id| b.first(id, honest).map(|e| &e[2..]);
    let Some(info) = parse_beacon(f) else {
        // No SSID, or one longer than an SSID can be, is no beacon.
        assert!(first(0).is_none_or(|s| s.len() > 32), "no beacon, yet an SSID");
        return;
    };
    assert_eq!(Some(info.ssid), first(0));
    assert!(within(f, info.ssid));
    assert_eq!(info.channel, first(3).and_then(|d| d.first().copied()).unwrap_or(0));
    assert_eq!(info.rsn, first(48).is_some());
    assert_eq!(info.bssid[..], f[16..22]);
    assert_eq!(info.capability, u16::from_le_bytes([f[34], f[35]]));
}

#[test]
fn beacons_read_as_built_before_and_after_a_lie() {
    each(|rng| {
        let mut b = build(rng);
        let subtype = rng.pick(&[5u8, 8]);
        let fixed = rng.bytes(12);
        let f = mgmt(rng, 0, subtype, &[&fixed[..], &b.tags].concat());
        check(&f, &b, b.at.len());
        assert!(parse_beacon(&mgmt(rng, 0, 4, &f[24..])).is_none(), "a probe request");
        assert!(parse_beacon(&mgmt(rng, 2, subtype, &f[24..])).is_none(), "a data frame");
        assert!(parse_beacon(&f[..rng.below(36)]).is_none(), "no room for the fixed fields");
        let k = rng.below(b.at.len());
        let start = b.at[k].1;
        let room = b.tags.len() - start - 2;
        if room < 255 {
            b.tags[start + 1] = (room + 1 + rng.below(255 - room)) as u8;
            let f = mgmt(rng, 0, subtype, &[&fixed[..], &b.tags].concat());
            check(&f, &b, k);
        }
    });
}
