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

//! The hello offers X25519 and secp256r1, each with its share, and a server's
//! share is taken only in an offered group at that group's exact size.

use crate::client_hello::client_hello;
use crate::server_share::{parse, ServerShare};

fn holds(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

#[test]
fn the_hello_names_both_groups_and_carries_a_share_for_each() {
    let hello = client_hello(b"relay", &[1; 32], &[2; 32], &[0xAA; 32], &[0x04; 65]);
    assert!(holds(&hello, &[0, 10, 0, 6, 0, 4, 0x00, 0x1d, 0x00, 0x17]), "supported_groups");
    let mut share = vec![0, 51, 0, 107, 0, 105, 0x00, 0x1d, 0, 32];
    share.extend_from_slice(&[0xAA; 32]);
    share.extend_from_slice(&[0x00, 0x17, 0, 65]);
    share.extend_from_slice(&[0x04; 65]);
    assert!(holds(&hello, &share), "key_share with both entries, x25519 first");
}

fn body(group: u16, key: &[u8]) -> Vec<u8> {
    let mut out = group.to_be_bytes().to_vec();
    out.extend_from_slice(&(key.len() as u16).to_be_bytes());
    out.extend_from_slice(key);
    out
}

#[test]
fn a_p256_share_is_taken_only_as_an_uncompressed_point() {
    let mut point = [7u8; 65];
    point[0] = 4;
    assert_eq!(parse(&body(0x0017, &point)), Some(ServerShare::Secp256r1(point)));
    point[0] = 3;
    assert_eq!(parse(&body(0x0017, &point)), None, "a 65 byte point not tagged 04");
    assert_eq!(parse(&body(0x0017, &[2; 33])), None, "a compressed point");
    assert_eq!(parse(&body(0x0017, &[4; 32])), None, "an x25519 sized key");
}

#[test]
fn a_group_this_client_did_not_offer_is_refused() {
    assert_eq!(parse(&body(0x0018, &[4; 97])), None, "secp384r1");
    assert_eq!(parse(&body(0x001e, &[9; 56])), None, "x448");
    assert_eq!(parse(&body(0x001d, &[9; 32])), Some(ServerShare::X25519([9; 32])));
}
