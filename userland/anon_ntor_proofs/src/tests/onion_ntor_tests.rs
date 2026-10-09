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
//! Join.
//! hs-ntor and the cells it travels in, held to vectors/hs_ntor_vector.expect:
//! the formulas of the fork's src/test/hs_ntor_ref.py over fixed scalars,
//! cross-checked against that reference module itself when generated.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::hex;
use crate::onion::cells::{introduce1, introduce_ack, link_specifiers, rendezvous2};
use crate::onion::hs_ntor::{intro_keys, rend_keys};

const EXPECT: &str = include_str!("../../vectors/hs_ntor_vector.expect");

fn vector() -> BTreeMap<String, Vec<u8>> {
    EXPECT
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(k, v)| (String::from(k), hex(v.trim())))
        .collect()
}

fn k(v: &BTreeMap<String, Vec<u8>>, name: &str) -> [u8; 32] {
    v[name].clone().try_into().expect("32 bytes")
}

const AUTH: [u8; 32] = [0x41; 32];
const SUB: [u8; 32] = [0x42; 32];

#[test]
fn intro_keys_match_the_reference() {
    let v = vector();
    let keys = intro_keys(&k(&v, "dh_bx"), &AUTH, &k(&v, "X"), &k(&v, "B"), &SUB);
    assert_eq!(keys.enc.to_vec(), v["enc_key"]);
    assert_eq!(keys.mac.to_vec(), v["mac_key"]);
}

#[test]
fn rendezvous_keys_match_the_reference_and_expand_to_the_hop_keys() {
    let v = vector();
    let hop = rend_keys(&k(&v, "dh_yx"), &k(&v, "dh_bx"), &AUTH, &k(&v, "B"), &k(&v, "X"), &k(&v, "Y"), &k(&v, "auth_mac"))
        .expect("the service's MAC matches");
    assert_eq!(hop.to_vec(), v["expanded"]);
}

#[test]
fn a_wrong_server_mac_or_a_zero_exchange_is_refused() {
    let v = vector();
    let mut bad = k(&v, "auth_mac");
    bad[31] ^= 1;
    assert!(rend_keys(&k(&v, "dh_yx"), &k(&v, "dh_bx"), &AUTH, &k(&v, "B"), &k(&v, "X"), &k(&v, "Y"), &bad).is_none());
    assert!(rend_keys(&[0; 32], &k(&v, "dh_bx"), &AUTH, &k(&v, "B"), &k(&v, "X"), &k(&v, "Y"), &k(&v, "auth_mac")).is_none());
    assert!(rend_keys(&k(&v, "dh_yx"), &[0; 32], &AUTH, &k(&v, "B"), &k(&v, "X"), &k(&v, "Y"), &k(&v, "auth_mac")).is_none());
    /* Y swapped for X: the MAC binds which key is whose. */
    assert!(rend_keys(&k(&v, "dh_yx"), &k(&v, "dh_bx"), &AUTH, &k(&v, "B"), &k(&v, "Y"), &k(&v, "X"), &k(&v, "auth_mac")).is_none());
}

#[test]
fn introduce1_matches_the_reference_byte_for_byte() {
    let v = vector();
    let keys = intro_keys(&k(&v, "dh_bx"), &AUTH, &k(&v, "X"), &k(&v, "B"), &SUB);
    let mut specs = alloc::vec![2u8, 0, 6, 10, 0, 0, 1, 0x23, 0x29, 2, 20];
    specs.extend_from_slice(&[0x53; 20]);
    let cell = introduce1(&AUTH, &k(&v, "X"), &keys, &[0x51; 20], (&[0x52; 32], &specs), None).expect("fits");
    assert_eq!(cell, v["introduce1"]);
    /* 56 cleartext and 190 padded plaintext make 246, plus X and the MAC. */
    assert_eq!(cell.len(), 246 + 64);
}

#[test]
fn introduce1_that_cannot_fit_is_refused() {
    let v = vector();
    let keys = intro_keys(&k(&v, "dh_bx"), &AUTH, &k(&v, "X"), &k(&v, "B"), &SUB);
    assert!(introduce1(&AUTH, &k(&v, "X"), &keys, &[1; 20], (&[2; 32], &[0u8; 400]), None).is_none());
}

#[test]
fn acks_and_rendezvous2_are_read_strictly() {
    assert_eq!(introduce_ack(&[0, 0, 0]), Some(0));
    assert_eq!(introduce_ack(&[0, 1, 0]), Some(1));
    assert_eq!(introduce_ack(&[0, 0]), None);
    let mut r = [0u8; 64];
    r[0] = 7;
    r[63] = 9;
    let (y, mac) = rendezvous2(&r).expect("64 bytes");
    assert_eq!((y[0], mac[31]), (7, 9));
    assert!(rendezvous2(&r[..63]).is_none());
}

#[test]
fn link_specifiers_round_trip_through_the_descriptor_reader() {
    let specs = link_specifiers([5, 6, 7, 8], 443, &[9; 20], &[10; 32]);
    assert_eq!(crate::onion::desc::intro::link_specifiers(&specs), Some(([5, 6, 7, 8], 443, [9; 20], [10; 32])));
}

/*
 * The virtual hop: the 128 hop key bytes above build it, a BEGIN sealed
 * to it matches the reference byte for byte, and the service's CONNECTED
 * opens on it, with the SENDME digest it records.
 */
#[test]
fn the_virtual_hop_seals_and_opens_as_the_reference_does() {
    use crate::circuit::{open, seal, Hop};
    let v = vector();
    let keys: [u8; 128] = v["expanded"].clone().try_into().unwrap();

    let mut hops = alloc::vec![Hop::onion(&keys)];
    let mut payload = [0u8; 509];
    payload[0] = 1;
    payload[3..5].copy_from_slice(&1u16.to_be_bytes());
    payload[9..11].copy_from_slice(&4u16.to_be_bytes());
    payload[11..15].copy_from_slice(b":80\0");
    seal(&mut hops, 0, &mut payload).expect("hop 0");
    assert_eq!(payload.to_vec(), v["hop_begin"]);

    let mut back: [u8; 509] = v["hop_connected"].clone().try_into().unwrap();
    let opened = open(&mut hops, &mut back).expect("the service's cell opens");
    assert_eq!(opened.hop, 0);
    assert_eq!(back[0], 4);
    assert_eq!(hops[0].last_seen.to_vec(), v["hop_connected_digest"]);
}

/* Behind three relay hops the service's hop is the fourth, and a cell from
 * it is told apart from one from the rendezvous point. */
#[test]
fn the_virtual_hop_rides_behind_three_relay_hops() {
    use super::onion_fixture::chain;
    use crate::circuit::{open, seal, Hop};
    let v = vector();
    let keys: [u8; 128] = v["expanded"].clone().try_into().unwrap();
    let mut client = chain();
    client.push(Hop::onion(&keys));
    let mut relays = chain();

    let mut payload = [0u8; 509];
    payload[0] = 1;
    payload[3..5].copy_from_slice(&1u16.to_be_bytes());
    payload[9..11].copy_from_slice(&4u16.to_be_bytes());
    payload[11..15].copy_from_slice(b":80\0");
    seal(&mut client, 3, &mut payload).expect("hop 3");
    for hop in relays.iter_mut() {
        hop.forward.apply(&mut payload[..]);
    }
    let want: [u8; 509] = v["hop_begin"].clone().try_into().unwrap();
    assert_eq!(payload, want, "the three relays peel down to the service's layer");

    let mut back: [u8; 509] = v["hop_connected"].clone().try_into().unwrap();
    for index in (0..3).rev() {
        relays[index].backward.apply(&mut back[..]);
    }
    assert_eq!(open(&mut client, &mut back).expect("opens").hop, 3);
}

#[test]
fn an_onion_begin_names_no_host_and_carries_no_flags() {
    use crate::onion::cells::begin_onion;
    assert_eq!(begin_onion(80), b":80\0".to_vec());
    assert_eq!(begin_onion(443), b":443\0".to_vec());
    assert_eq!(begin_onion(1), b":1\0".to_vec());
    assert_eq!(begin_onion(65535), b":65535\0".to_vec());
}

#[test]
fn only_the_three_rendezvous_replies_go_to_a_lookup() {
    use crate::onion::cells::is_onion_reply;
    let replies: Vec<u8> = (0..=255u8).filter(|c| is_onion_reply(*c)).collect();
    assert_eq!(replies, [37, 39, 40]);
}
