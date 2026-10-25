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

//! What a list lets a guest reach.

use super::dests_write::{entry, list, name, v4};
use crate::dests::{allows, check, Target as T};

#[test]
fn an_ipv4_network_covers_its_hosts_on_its_ports() {
    let l = list(vec![v4(24, (8000, 8080), [10, 1, 2, 0])]);
    assert_eq!(check(&l), Ok(1));
    assert!(allows(&l, T::V4([10, 1, 2, 77]), 8000) && allows(&l, T::V4([10, 1, 2, 255]), 8080));
    assert!(!allows(&l, T::V4([10, 1, 3, 1]), 8000));
    assert!(!allows(&l, T::V4([10, 1, 2, 1]), 7999) && !allows(&l, T::V4([10, 1, 2, 1]), 8081));
}

#[test]
fn nothing_matches_across_kinds() {
    let l = list(vec![v4(0, (1, 65535), [0; 4])]);
    assert!(allows(&l, T::V4([192, 0, 2, 1]), 443));
    assert!(!allows(&l, T::V6([0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), 443));
    assert!(!allows(&l, T::Name(b"example.org"), 443));
}

#[test]
fn ipv4_written_as_ipv6_is_matched_as_ipv4() {
    let l = list(vec![v4(32, (443, 443), [192, 0, 2, 7])]);
    let mapped = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 192, 0, 2, 7];
    assert!(allows(&l, T::V6(mapped), 443));
    let six = list(vec![entry(6, 0, (1, 65535), &[0; 16])]);
    assert!(!allows(&six, T::V6(mapped), 443));
}

#[test]
fn an_ipv6_network_covers_its_hosts() {
    let mut net = [0u8; 16];
    net[..4].copy_from_slice(&[0x20, 0x01, 0x0d, 0xb8]);
    let l = list(vec![entry(6, 32, (443, 443), &net)]);
    let mut host = net;
    host[15] = 9;
    assert!(allows(&l, T::V6(host), 443));
    host[3] = 0xb9;
    assert!(!allows(&l, T::V6(host), 443));
}

#[test]
fn a_name_matches_itself_alone() {
    let l = list(vec![name("api.example.org", (443, 443))]);
    assert!(allows(&l, T::Name(b"api.example.org"), 443));
    assert!(allows(&l, T::Name(b"API.Example.ORG."), 443));
    assert!(!allows(&l, T::Name(b"example.org"), 443));
    assert!(!allows(&l, T::Name(b"x.api.example.org"), 443));
    assert!(!allows(&l, T::Name(b"api.example.org.."), 443));
}
