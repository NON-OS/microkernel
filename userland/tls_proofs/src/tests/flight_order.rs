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

//! One Certificate, then the CertificateVerify that signs with its key.
//!
//! A caller that authenticates the peer itself (the anon link pins the TLS
//! leaf against the relay's CERTS cell) trusts that the leaf it is handed is
//! the key that signed the handshake. A second Certificate message after the
//! signature would hand it a different one.

use super::rfc8448_flight::messages;
use crate::example_leaf::EXAMPLE_LEAF;
use crate::fixtures::rfc8448::SERVER_MESSAGES;
use build::{certificate_message, flight_of, verdict};

#[path = "flight_order_build.rs"]
mod build;

#[test]
fn the_trace_rebuilt_this_way_still_verifies() {
    let m = messages(SERVER_MESSAGES);
    let leaf = verdict(&flight_of(&[m[0], m[1], m[2]])).expect("the honest order verifies");
    assert_eq!(leaf, &m[1][4..], "the certificate handed back is the one that signed");
}

#[test]
fn a_certificate_after_the_signature_is_refused() {
    let m = messages(SERVER_MESSAGES);
    let swapped = certificate_message(EXAMPLE_LEAF);
    let flight = flight_of(&[m[0], m[1], m[2], &swapped]);
    assert!(verdict(&flight).is_none(), "the leaf would no longer be the signing key");
}

#[test]
fn a_second_certificate_before_the_signature_is_refused() {
    let m = messages(SERVER_MESSAGES);
    let first = certificate_message(EXAMPLE_LEAF);
    let flight = flight_of(&[m[0], &first, m[1], m[2]]);
    assert!(verdict(&flight).is_none(), "RFC 8446 allows the server one Certificate");
}

#[test]
fn a_second_signature_is_refused() {
    let m = messages(SERVER_MESSAGES);
    let flight = flight_of(&[m[0], m[1], m[2], m[2]]);
    assert!(verdict(&flight).is_none());
}
