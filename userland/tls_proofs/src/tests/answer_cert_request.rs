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

//! The one-shot reply owes a server that asked for a certificate the same
//! closing flight the session path sends: an empty Certificate, then a
//! Finished over the transcript through it, then the request.

use super::answer_rfc8448::request;
use super::rfc8448_flight::{coalesced, keyed};
use crate::handshake_state::Progress;

fn done() -> crate::server_complete::ServerComplete {
    let flight = coalesced();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    state.verify(b"", 0, false).expect("verifies")
}

#[test]
fn a_one_shot_reply_to_a_certificate_request_sends_the_empty_certificate_first() {
    let mut done = done();
    let certificate = alloc::vec![11u8, 0, 0, 4, 0, 0, 0, 0];
    let mut finished_hash = done.finished_hash;
    finished_hash[0] ^= 0x5a;
    done.client_certificate = Some(certificate.clone());
    done.finished_hash = finished_hash;
    let keys = done.handshake;
    let answer = crate::handshake_state::reply(done, &request()).expect("reply");

    let len = usize::from(u16::from_be_bytes([answer.flight[3], answer.flight[4]]));
    let record = &answer.flight[..5 + len];
    let plain = crate::record_open::open(keys.suite, &keys.client_key, &keys.client_iv, 0, record)
        .expect("sealed under the client handshake key");
    let (inner, kind) = crate::inner_plain::split(&plain).expect("inner plaintext");
    assert_eq!(kind, 22);
    assert_eq!(&inner[..8], &certificate[..], "the empty Certificate leads");
    let mac =
        crate::finished_value::finished_value(&keys.client_secret, &finished_hash).expect("mac");
    assert_eq!(&inner[8..12], &[20, 0, 0, 32]);
    assert_eq!(&inner[12..], &mac, "the Finished covers the transcript through it");

    let data = &answer.flight[5 + len..];
    let first = crate::record_open::open(
        answer.app.suite,
        &answer.app.client_key,
        &answer.app.client_iv,
        0,
        data,
    )
    .expect("the request is sealed under the client application key at sequence 0");
    let (inner, kind) = crate::inner_plain::split(&first).expect("inner plaintext");
    assert_eq!((kind, inner), (23, &request()[..]));
}

#[test]
fn without_a_request_the_one_shot_flight_is_unchanged() {
    let done = done();
    assert!(done.client_certificate.is_none());
    let expected = crate::client_finished::client_reply(&done).expect("fin");
    let answer = crate::handshake_state::reply(done, &request()).expect("reply");
    assert_eq!(&answer.flight[..expected.len()], &expected[..]);
}
