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

//! A network's address: the same each time this boot, another for any
//! other network or boot, and always locally administered and individual.

use nonos_wifi_core::address::{local_unicast, security_of, Addresses};
use nonos_wifi_core::address::{SECURITY_OPEN, SECURITY_PASSPHRASE as PSK, SECURITY_WPA3};

pub fn key(b: u8) -> Addresses {
    Addresses::new(|k| {
        k.fill(b);
        true
    })
    .unwrap()
}

fn mine(a: [u8; 6]) -> bool {
    a[0] & 0x03 == 0x02
}

#[test]
fn one_network_keeps_its_address_and_others_get_their_own() {
    let a = key(1);
    assert_eq!(a.join(b"home", PSK), a.join(b"home", PSK), "a reconnect keeps it");
    let all = [
        a.join(b"home", PSK),
        a.join(b"work", PSK),
        a.join(b"home", SECURITY_WPA3),
        a.join(b"home", SECURITY_OPEN),
        a.join(b"hom", PSK),
        a.join(b"home\x01", SECURITY_OPEN),
    ];
    for (i, x) in all.iter().enumerate() {
        assert!(all[i + 1..].iter().all(|y| y != x), "{i} repeats");
    }
    assert_ne!(key(2).join(b"home", PSK), a.join(b"home", PSK), "another boot, another address");
}

#[test]
fn every_address_is_locally_administered_and_individual() {
    let a = key(3);
    for n in 0..=255u8 {
        assert!(mine(a.join(&[n; 7], PSK)), "{n}");
    }
    assert_eq!(local_unicast([0xff; 6])[0], 0xfe);
    assert_eq!(local_unicast([0x00; 6])[0], 0x02);
}

#[test]
fn without_randomness_there_is_no_secret() {
    assert!(Addresses::new(|_| false).is_none());
}

#[test]
fn the_security_mode_is_the_requests() {
    assert_eq!(security_of(false, b""), SECURITY_OPEN);
    assert_eq!(security_of(true, b""), SECURITY_OPEN);
    assert_eq!(security_of(false, b"pass"), PSK);
    assert_eq!(security_of(true, b"pass"), SECURITY_WPA3);
}
