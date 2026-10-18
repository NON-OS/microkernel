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

//! A scan's address: fresh each scan, never the last one, never the
//! network's, and none at all without randomness.

use super::key_tests::key;
use nonos_wifi_core::address::SECURITY_PASSPHRASE as PSK;

#[test]
fn scans_never_repeat_and_never_take_the_networks_address() {
    let mut a = key(4);
    let net = a.join(b"home", PSK);
    let mut n = 0u8;
    let mut last = None;
    for _ in 0..200 {
        // A poor source: each value comes twice running.
        let s = a.scan(net, |b| {
            b.fill(n / 2);
            n = n.wrapping_add(1);
            true
        });
        let s = s.expect("a fresh address");
        assert!(Some(s) != last && s != net && s[0] & 0x03 == 0x02);
        last = Some(s);
    }
}

#[test]
fn a_draw_equal_to_the_networks_address_is_drawn_again() {
    let mut a = key(5);
    let net = a.join(b"home", PSK);
    let mut draws = [net, [0x42; 6]].into_iter();
    let s = a.scan(net, |b| {
        b.copy_from_slice(&draws.next().unwrap());
        true
    });
    assert_eq!(s, Some([0x42, 0x42, 0x42, 0x42, 0x42, 0x42]));
}

#[test]
fn randomness_that_only_repeats_or_fails_gives_no_address() {
    let mut a = key(6);
    let net = a.join(b"home", PSK);
    let same = |b: &mut [u8]| {
        b.fill(0x24);
        true
    };
    assert!(a.scan(net, same).is_some());
    assert!(a.scan(net, same).is_none(), "only the last one, over and over");
    assert!(a.scan(net, |_| false).is_none());
}
