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

//! The flight a re-signing gateway sends is refused in one pass.

use alloc::vec::Vec;

use super::rfc8448_flight::{keyed, sealed};
use crate::fixtures::certs::GATEWAY;
use crate::fixtures::rfc8448::SERVER_HELLO_RECORD;
use crate::handshake_state::{Progress, Refusal};

/* 2026-10-01, inside the leaf's month of validity. */
const NOW: u64 = 20261001000000;

fn message(kind: u8, body: &[u8]) -> Vec<u8> {
    let len = (body.len() as u32).to_be_bytes();
    [&[kind], &len[1..], body].concat()
}

/// EncryptedExtensions, the gateway chain, a CertificateVerify, and a
/// Finished whose MAC is right, all in one record as the capture had it.
fn gateway_flight() -> Vec<u8> {
    let mut list = Vec::new();
    for der in GATEWAY {
        list.extend_from_slice(&(der.len() as u32).to_be_bytes()[1..]);
        list.extend_from_slice(der);
        list.extend_from_slice(&[0, 0]);
    }
    let body = [&[0u8][..], &(list.len() as u32).to_be_bytes()[1..], &list].concat();
    let signature = [&[0x08, 0x04, 0x01, 0x00][..], &[0x5a; 256]].concat();
    let parts = [message(8, &[0, 0]), message(11, &body), message(15, &signature)];
    let state = keyed(SERVER_HELLO_RECORD);
    let mut transcript = state.transcript.clone();
    parts.iter().for_each(|m| transcript.push(m));
    let secret = &state.keys.server_secret;
    let mac = crate::finished_value::finished_value(secret, &transcript.digest()).expect("mac");
    let all = [&parts[0][..], &parts[1], &parts[2], &message(20, &mac)].concat();
    sealed(&[&all], 22)
}

#[test]
fn the_gateway_chain_is_refused_for_one_agreement_and_two_signatures() {
    let flight = gateway_flight();
    nonos_libc::reset();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    let refused = state.answer(b"nonos.software", NOW, b"GET / HTTP/1.1\r\n\r\n").err();
    assert_eq!(refused, Some(Refusal::Unverified));
    let c = nonos_libc::counts();
    assert_eq!(c.kernel_round_trips, 1, "the X25519 agreement, nothing else");
    let ipc = c.rsa + c.p256 + c.p384 + c.sha384 + c.other_ipc;
    assert!(ipc <= 2, "two chain links at most, got {ipc}");
}

#[test]
fn with_the_chain_set_aside_the_forged_signature_still_fails() {
    let flight = gateway_flight();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    assert!(state.verify(b"nonos.software", NOW, false).is_none());
}
