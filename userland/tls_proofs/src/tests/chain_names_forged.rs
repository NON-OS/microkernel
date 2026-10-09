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

//! The names a certificate claims are the ones in its subjectAltName extension,
//! read as the structure it is, and nothing else in the certificate.

use super::der_build::{certificate, general_names, key_holding, tlv};
use crate::cert_dns_match::matches;
use crate::example_leaf::EXAMPLE_LEAF;
use crate::fixtures::certs::P384_MANY_NAMES;

const DNS: u8 = 0x82;
const URI: u8 = 0x86;

/*
 * A certificate legitimately issued for attacker.example whose RSA key carries
 * the bytes of a second subjectAltName naming the victim. The key comes before
 * the extensions, so a search for the extension's OID bytes finds the copy in
 * the key first and believes the name in it. The CA signed the key as sent;
 * every signature in such a chain verifies.
 */
fn forged() -> Vec<u8> {
    let fake = tlv(0x04, &general_names(&[(DNS, b"victim.com")]));
    let mut planted = vec![0x06, 0x03, 0x55, 0x1d, 0x11];
    planted.extend_from_slice(&fake);
    certificate(&key_holding(&planted), &general_names(&[(DNS, b"attacker.example")]))
}

#[test]
fn a_name_hidden_in_the_public_key_is_not_claimed() {
    let cert = forged();
    assert!(!matches(&cert, b"victim.com"), "the key's bytes are not an extension");
    assert!(matches(&cert, b"attacker.example"), "the real extension still answers");
}

/*
 * A URI entry longer than 127 bytes has a long-form length. Stepping over it a
 * byte at a time walks into its value, where chosen bytes read as a dNSName.
 */
#[test]
fn a_name_inside_a_long_entry_is_not_a_dns_name() {
    let mut uri = vec![DNS, 10];
    uri.extend_from_slice(b"victim.com");
    uri.resize(200, b'a');
    let san = general_names(&[(URI, &uri), (DNS, b"attacker.example")]);
    let cert = certificate(&key_holding(b"plain key"), &san);
    assert!(!matches(&cert, b"victim.com"));
    assert!(matches(&cert, b"attacker.example"), "the entry after the long one is reached");
}

#[test]
fn a_dns_name_longer_than_127_bytes_is_read_whole() {
    let mut long = Vec::new();
    for _ in 0..4 {
        long.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz0123456789.");
    }
    long.extend_from_slice(b"example");
    assert!(long.len() > 127);
    let cert = certificate(&key_holding(b"k"), &general_names(&[(DNS, &long)]));
    assert!(matches(&cert, &long));
    assert!(!matches(&cert, b"example"));
}

#[test]
fn every_name_of_a_certificate_with_many_matches() {
    for i in 0..64 {
        let host = format!("host{i:03}.many.proof.nonos.test");
        assert!(matches(P384_MANY_NAMES, host.as_bytes()), "{host}");
    }
    assert!(!matches(P384_MANY_NAMES, b"host064.many.proof.nonos.test"));
}

/*
 * The structural walk needs the whole certificate, so no cut of one claims a
 * name. The search it replaced still answered for any cut that kept the
 * extension.
 */
#[test]
fn no_prefix_of_a_certificate_claims_a_name() {
    for cut in 0..EXAMPLE_LEAF.len() {
        assert!(!matches(&EXAMPLE_LEAF[..cut], b"example.com"), "cut at {cut}");
    }
    assert!(matches(EXAMPLE_LEAF, b"example.com"));
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/*
 * Damaged certificates and random bytes never panic the name check. A damaged
 * certificate may still parse and match, which is not a fault: what admits a
 * certificate is the signature over it, checked separately.
 */
#[test]
fn damaged_certificates_and_noise_never_panic() {
    let seeds: [Vec<u8>; 3] = [EXAMPLE_LEAF.to_vec(), forged(), P384_MANY_NAMES.to_vec()];
    let mut s = 0x9E37_79B9u32;
    for round in 0..120_000u32 {
        let mut cert = seeds[(round % 3) as usize].clone();
        for _ in 0..1 + xorshift(&mut s) % 4 {
            let at = xorshift(&mut s) as usize % cert.len();
            cert[at] = xorshift(&mut s) as u8;
        }
        let _ = matches(&cert, b"example.com");
        let noise: Vec<u8> = (0..xorshift(&mut s) % 96).map(|_| xorshift(&mut s) as u8).collect();
        let _ = matches(&noise, b"example.com");
    }
}
