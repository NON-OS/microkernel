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
//! Which authority certificates are still to be fetched.
//!
//! ek's boot held three certificates against a quorum of four and failed
//! every consensus with "held certs and needed 3 4": the first sweep ended at
//! the first certificate and the other four were never asked for again.

use alloc::vec::Vec;

use crate::cert_rule::{enough, unheld, wanted};
use crate::directory::authority::{AUTHORITIES, REQUIRED_SIGNATURES};
use crate::directory::cert::parse as parse_cert;
use crate::directory::consensus::parse;
use crate::sha1::Sha1;
use crate::vectors::CONSENSUS;

const CERT: &[u8] = include_bytes!("../../vectors/authority-cert.txt");

fn identities() -> Vec<[u8; 20]> {
    AUTHORITIES.iter().map(|a| a.v3ident).collect()
}

fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h = Sha1::new();
    h.update(data);
    h.finish()
}

/// The (identity, signing key) every sha256 line of the live consensus names.
fn named() -> Vec<([u8; 20], [u8; 20])> {
    let doc = parse(CONSENSUS).expect("the live consensus parses");
    doc.signatures.iter().filter(|s| s.sha256).map(|s| (s.identity, s.signing_key)).collect()
}

#[test]
fn three_certificates_are_not_a_quorum_of_seven() {
    assert_eq!(REQUIRED_SIGNATURES, 4);
    assert!(!enough(3, REQUIRED_SIGNATURES), "the boot that stalled must keep sweeping");
    assert!(enough(4, REQUIRED_SIGNATURES));
    assert!(enough(7, REQUIRED_SIGNATURES));
    assert!(!enough(0, REQUIRED_SIGNATURES));
}

#[test]
fn a_sweep_asks_only_the_authorities_not_held() {
    let held = [4usize, 5, 6];
    let asked: Vec<usize> = (0..AUTHORITIES.len()).filter(|i| wanted(*i, &held, &[])).collect();
    assert_eq!(asked, [0, 1, 2, 3]);
    let all: Vec<usize> = (0..AUTHORITIES.len()).collect();
    assert!((0..AUTHORITIES.len()).all(|i| !wanted(i, &all, &[])), "a full set asks nobody");
}

#[test]
fn a_held_certificate_named_for_refetch_is_asked_again() {
    let all: Vec<usize> = (0..AUTHORITIES.len()).collect();
    let asked: Vec<usize> = (0..AUTHORITIES.len()).filter(|i| wanted(*i, &all, &[2, 5])).collect();
    assert_eq!(asked, [2, 5]);
}

#[test]
fn the_live_consensus_names_all_seven_authorities() {
    let named = named();
    assert_eq!(named.len(), 7);
    let none = unheld(&named, &[], &identities());
    assert_eq!(none, [0, 1, 2, 3, 4, 5, 6], "with nothing held, every authority is fetched");
}

/*
 * The real certificate of authority 0 (ATORDAeuclive) carries the signing key
 * whose digest the live consensus names for it, FD7D72F7...; held, it is not
 * fetched again, and the six others still are.
 */
#[test]
fn a_held_certificate_with_the_named_key_is_not_fetched_again() {
    let cert = parse_cert(CERT).expect("the authority certificate parses");
    let digest = sha1(&cert.signing_pkcs1);
    assert_eq!(digest.to_vec(), crate::hex("fd7d72f78dd1dd2077899705fb3b171de5054800"));
    assert_eq!(unheld(&named(), &[(0, digest)], &identities()), [1, 2, 3, 4, 5, 6]);
}

#[test]
fn a_certificate_whose_key_was_rotated_is_fetched_again() {
    let retired = [0x11u8; 20];
    assert_eq!(unheld(&named(), &[(0, retired)], &identities())[0], 0);
}

#[test]
fn every_named_key_held_leaves_nothing_to_fetch() {
    let named = named();
    let ids = identities();
    let held: Vec<(usize, [u8; 20])> = named
        .iter()
        .map(|(id, key)| (ids.iter().position(|k| k == id).expect("known"), *key))
        .collect();
    assert!(unheld(&named, &held, &ids).is_empty());
}

#[test]
fn a_line_for_an_unknown_authority_is_ignored_and_duplicates_count_once() {
    let ids = identities();
    let named = [([0xEEu8; 20], [1u8; 20]), (ids[2], [2u8; 20]), (ids[2], [3u8; 20])];
    assert_eq!(unheld(&named, &[], &ids), [2]);
}
