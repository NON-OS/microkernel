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

//! What the captured certificates actually contain.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use crate::link::certs::split;
use crate::link::constants::{CERT_ED_ID_SIGN, CERT_ED_SIGN_LINK};
use crate::link::ed_cert::parse;
use crate::vectors::{CERTS, IDENTITY};

/// The relay sent five certificates: RSA link, RSA identity, the two Ed25519
#[test]
fn the_cell_carries_five_certificates() {
    let types: Vec<u8> = split(CERTS).expect("cell walks").map(|entry| entry.cert_type).collect();
    assert_eq!(types, vec![1, 2, 4, 5, 7]);
}

#[test]
fn the_signing_certificate_names_the_identity_in_an_extension() {
    let entry = split(CERTS)
        .expect("walks")
        .find(|e| e.cert_type == CERT_ED_ID_SIGN)
        .expect("type 4 present");
    let cert = parse(entry.body).expect("parses");
    assert_eq!(cert.cert_type, CERT_ED_ID_SIGN);
    assert_eq!(cert.signed_with, Some(IDENTITY));
}

#[test]
fn the_link_certificate_certifies_a_digest() {
    let entry = split(CERTS)
        .expect("walks")
        .find(|e| e.cert_type == CERT_ED_SIGN_LINK)
        .expect("type 5 present");
    let cert = parse(entry.body).expect("parses");
    assert_eq!(cert.cert_type, CERT_ED_SIGN_LINK);
    assert!(cert.signed_with.is_none(), "type 5 carries no signing key extension");
    let digest = crate::crypto::sha256(crate::vectors::LEAF).expect("digest");
    assert_eq!(cert.certified_key, digest);
}

#[test]
fn the_signed_region_stops_before_the_signature() {
    for entry in split(CERTS).expect("walks") {
        if let Some(cert) = parse(entry.body) {
            assert_eq!(cert.signed.len(), entry.body.len() - 64);
        }
    }
}
