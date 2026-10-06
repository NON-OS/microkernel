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
//! Decoding a descriptor. The fixture is vectors/onion_descriptor.txt,
//! built by vectors/onion_descriptor.py the way the fork's hs_descriptor.c
//! encodes one, from fixed test seeds; tor_desc_bad_sig.txt is the fork's
//! own HS_DESC_BAD_SIG from src/test/test_hs_descriptor.inc.

use alloc::vec::Vec;

use crate::hex;
use crate::onion::cert::{check, TYPE_DESC_SIGNING};
use crate::onion::desc::intro::link_specifiers;
use crate::onion::desc::{decode, DescError};

const DESC: &[u8] = include_bytes!("../../vectors/onion_descriptor.txt");
const TOR_BAD_SIG: &[u8] = include_bytes!("../../vectors/tor_desc_bad_sig.txt");
const NOW: u64 = 1_790_000_000;

fn blinded() -> [u8; 32] {
    hex("03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8").try_into().unwrap()
}

const SUB: [u8; 32] = [0x55; 32];

#[test]
fn the_fixture_decodes_to_its_one_introduction_point() {
    let desc = decode(DESC, &blinded(), &SUB, NOW, |_| None).expect("decodes");
    assert_eq!(desc.intro_points.len(), 1);
    let ip = &desc.intro_points[0];
    assert_eq!(ip.address, [1, 2, 3, 4]);
    assert_eq!(ip.port, 9001);
    assert_eq!(ip.rsa_identity, [0xAA; 20]);
    assert_eq!(ip.ed25519_identity, [0xBB; 32]);
    assert_eq!(ip.onion_key, [0x66; 32]);
    assert_eq!(ip.enc_key, [0x77; 32]);
    assert_eq!(ip.auth_key.to_vec(), hex("a09aa5f47a6759802ff955f8dc2d2a14a5c99d23be97f864127ff9383455a4f0"));
}

#[test]
fn another_services_blinded_key_is_refused() {
    let mut other = blinded();
    other[0] ^= 1;
    assert_eq!(decode(DESC, &other, &SUB, NOW, |_| None).err(), Some(DescError::WrongService));
}

#[test]
fn a_wrong_subcredential_does_not_open_the_layers() {
    assert_eq!(decode(DESC, &blinded(), &[0x56; 32], NOW, |_| None).err(), Some(DescError::Layer));
}

#[test]
fn an_expired_signing_certificate_is_refused() {
    /* The fixture's certificates expire at hour 600000. */
    assert_eq!(decode(DESC, &blinded(), &SUB, 600_000 * 3600, |_| None).err(), Some(DescError::Certificate));
}

/*
 * Any byte changed in the signed part fails the signature, or the
 * certificate, or the layer MAC, or the parse; none of them decodes.
 */
#[test]
fn a_changed_byte_anywhere_is_refused() {
    let text = core::str::from_utf8(DESC).unwrap();
    let rev = text.find("revision-counter 42").unwrap() + 17;
    let mut changed = DESC.to_vec();
    changed[rev] = b'3';
    assert_eq!(decode(&changed, &blinded(), &SUB, NOW, |_| None).err(), Some(DescError::Signature));

    for at in (0..DESC.len()).step_by(97) {
        let mut flipped: Vec<u8> = DESC.to_vec();
        flipped[at] ^= 0x01;
        assert!(decode(&flipped, &blinded(), &SUB, NOW, |_| None).is_err(), "byte {at}");
    }
}

#[test]
fn lines_after_the_signature_are_refused() {
    let mut longer = DESC.to_vec();
    longer.extend_from_slice(b"introduction-point AAAA\n");
    assert_eq!(decode(&longer, &blinded(), &SUB, NOW, |_| None).err(), Some(DescError::Malformed));
}

#[test]
fn an_oversized_descriptor_is_refused_before_parsing() {
    let mut big = DESC.to_vec();
    big.resize(50_001, b'\n');
    assert_eq!(decode(&big, &blinded(), &SUB, NOW, |_| None).err(), Some(DescError::Malformed));
}

/// The fork's HS_DESC_BAD_SIG: its certificate verifies, its signature line
/// is malformed, and the descriptor is refused.
#[test]
fn the_forks_bad_signature_descriptor_is_refused() {
    let text = core::str::from_utf8(TOR_BAD_SIG).unwrap();
    let start = text.find("-----BEGIN ED25519 CERT-----\n").unwrap() + 29;
    let end = text.find("-----END ED25519 CERT-----").unwrap();
    let cert = crate::directory::base64::decode(&text.as_bytes()[start..end]).unwrap();
    let (_, signer) = check(&cert, TYPE_DESC_SIGNING, None, 0).expect("its certificate verifies");
    assert!(decode(TOR_BAD_SIG, &signer, &SUB, 0, |_| None).is_err());
}

#[test]
fn link_specifiers_need_all_three_kinds() {
    let mut all = alloc::vec![3u8, 0, 6, 9, 9, 9, 9, 0x23, 0x29, 2, 20];
    all.extend_from_slice(&[1u8; 20]);
    all.extend_from_slice(&[3u8, 32]);
    all.extend_from_slice(&[2u8; 32]);
    assert_eq!(link_specifiers(&all), Some(([9, 9, 9, 9], 9001, [1u8; 20], [2u8; 32])));
    assert_eq!(link_specifiers(&all[..all.len() - 1]), None, "truncated");
    let mut no_ed = all.clone();
    no_ed[0] = 2;
    assert_eq!(link_specifiers(&no_ed), None);
    let mut zero_port = all.clone();
    zero_port[7] = 0;
    zero_port[8] = 0;
    assert_eq!(link_specifiers(&zero_port), None);
}

#[test]
fn a_repeated_outer_keyword_is_refused() {
    let text = core::str::from_utf8(DESC).unwrap();
    let doubled = text.replacen("revision-counter 42\n", "revision-counter 42\nrevision-counter 42\n", 1);
    assert_eq!(decode(doubled.as_bytes(), &blinded(), &SUB, NOW, |_| None).err(), Some(DescError::Malformed));
}

#[test]
fn the_hsdir_request_names_the_blinded_key_unpadded() {
    use crate::onion::fetch::request;
    assert_eq!(
        request(&blinded()),
        b"GET /tor/hs/3/A6EHv/POEL4dcN0Y50vAmWfk1jCbpQ1fHdyGZBJVMbg HTTP/1.0\r\n\r\n".to_vec()
    );
}

#[test]
fn the_hsdir_answer_is_read_by_status() {
    use crate::onion::fetch::{body, Answer};
    let mut ok = b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\n\r\n".to_vec();
    ok.extend_from_slice(DESC);
    assert_eq!(body(&ok), Ok(DESC));
    assert_eq!(body(b"HTTP/1.0 404 Not found\r\n\r\n"), Err(Answer::NotFound));
    assert_eq!(body(b"HTTP/1.0 400 Bad request\r\n\r\n"), Err(Answer::Refused));
    assert_eq!(body(b"HTTP/1.0 200 OK\r\n"), Err(Answer::Partial));
    assert_eq!(body(b"HTTP/1.0 200 OK\r\n\r\n"), Err(Answer::Refused), "an empty body");
    let mut huge = b"HTTP/1.0 200 OK\r\n\r\n".to_vec();
    huge.resize(huge.len() + 50_001, b'x');
    assert_eq!(body(&huge), Err(Answer::Refused));
    assert_eq!(body(&[b'x'; 5000]), Err(Answer::Refused), "headers that never end");
}
