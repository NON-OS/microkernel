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
//! The input a network's address is keyed on reads back to one (SSID,
//! security) pair: the security octet is one fixed width and always last,
//! so no two pairs give the same bytes, with no length in them.

use nonos_wifi_core::address::{identity, local_unicast, Addresses};
use nonos_wifi_core::wpa::sha256::hmac_sha256;

fn bytes(ssid: &[u8], security: u8) -> Vec<u8> {
    identity(ssid, &security).concat()
}

/// The empty SSID, the longest (32 octets), and SSIDs whose last octet is
/// a security mode's value.
fn ssids() -> Vec<Vec<u8>> {
    let mut v: Vec<Vec<u8>> = vec![vec![], vec![0], vec![1], vec![2], b"a".to_vec()];
    v.extend([b"a\0".to_vec(), b"a\x01".to_vec(), b"a\x02".to_vec(), b"a\x01\x02".to_vec()]);
    v.extend([vec![b'x'; 31], vec![b'x'; 32], [vec![b'x'; 31], vec![2]].concat()]);
    v
}

#[test]
fn every_input_reads_back_to_its_own_pair_and_no_two_are_the_same() {
    let mut seen: Vec<Vec<u8>> = Vec::new();
    for s in ssids() {
        for m in 0..=2u8 {
            let b = bytes(&s, m);
            assert_eq!(b.len(), 4 + s.len() + 1, "the label, the SSID, one octet");
            assert_eq!(&b[..4], b"join");
            assert_eq!(b[b.len() - 1], m, "the security octet, last");
            assert_eq!(&b[4..b.len() - 1], &s[..], "and the SSID between");
            assert!(!seen.contains(&b), "{s:?} under {m} repeats an input");
            seen.push(b);
        }
    }
    assert_eq!(seen.len(), ssids().len() * 3);
}

#[test]
fn the_address_is_keyed_on_exactly_that_input() {
    let a = Addresses::new(|k| {
        *k = [7; 32];
        true
    })
    .unwrap();
    for s in ssids() {
        for m in 0..=2u8 {
            let mac = hmac_sha256(&[7; 32], &bytes(&s, m));
            assert_eq!(a.join(&s, m), local_unicast(mac[..6].try_into().unwrap()));
        }
    }
}
