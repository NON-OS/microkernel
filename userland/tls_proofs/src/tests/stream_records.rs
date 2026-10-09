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

//! The records a peer sends down an open session, against the session the
//! anon link holds: connect_unauthenticated, completed against a real server.

use super::fake_server::Wire;
use crate::stream::{connect_unauthenticated, Stream};

const APPLICATION_DATA: u8 = 23;
const HANDSHAKE: u8 = 22;

fn open() -> (Wire, Stream) {
    let mut wire = Wire::new();
    let stream = connect_unauthenticated(&mut wire, b"relay.example").unwrap_or_else(|_| panic!("handshake completes"));
    (wire, stream)
}

#[test]
fn a_session_opens_and_carries_data_both_ways() {
    let (mut wire, mut stream) = open();
    assert!(stream.leaf().is_some(), "the server's certificate is kept for the caller");
    wire.send_record(APPLICATION_DATA, b"hello");
    assert_eq!(stream.read(&mut wire).ok(), Some(b"hello".to_vec()));
    assert!(!stream.is_done());
    assert!(stream.write_all(&mut wire, b"reply").is_ok());
}

/*
 * RFC 8446 5.2: no protected record is longer than 2^14 + 256 bytes. A
 * header claiming more was never refused: the session waited for the rest
 * of a record no peer may send, never reported itself done, and a caller
 * polling it read nothing for ever.
 */
#[test]
fn a_record_longer_than_the_protocol_allows_ends_the_session() {
    let (mut wire, mut stream) = open();
    wire.send_raw(&[APPLICATION_DATA, 3, 3, 0x48, 0x00]);
    wire.send_raw(&[0u8; 100]);
    assert_eq!(stream.read(&mut wire).ok(), Some(Vec::new()));
    assert!(stream.is_done(), "the session ends at the oversized header");
}

/*
 * RFC 8446 5.2 and 5.4: a record's plaintext, content type included, is at
 * most 2^14 + 1 bytes. A peer that sealed more had all of it taken.
 */
#[test]
fn plaintext_longer_than_2_14_is_refused() {
    let (mut wire, mut stream) = open();
    wire.send_record(APPLICATION_DATA, &[7u8; (1 << 14) + 1]);
    assert_eq!(stream.read(&mut wire).ok(), Some(Vec::new()));
    assert!(stream.is_done());
}

#[test]
fn a_full_size_record_is_still_read() {
    let (mut wire, mut stream) = open();
    wire.send_record(APPLICATION_DATA, &[7u8; 1 << 14]);
    let got = stream.read(&mut wire).unwrap_or_else(|_| panic!("read"));
    assert_eq!(got.len(), 1 << 14);
    assert!(!stream.is_done());
}

#[test]
fn a_key_update_ends_the_session_and_a_ticket_does_not() {
    let (mut wire, mut stream) = open();
    wire.send_record(HANDSHAKE, &[4, 0, 0, 4, 0, 0, 0, 0]);
    wire.send_record(APPLICATION_DATA, b"after ticket");
    assert_eq!(stream.read(&mut wire).ok(), Some(b"after ticket".to_vec()));
    wire.send_record(HANDSHAKE, &[24, 0, 0, 1, 0]);
    let _ = stream.read(&mut wire);
    assert!(stream.is_done());
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/// Whatever bytes follow the handshake, the session never panics, never
/// hands back more than was sent, and stops at the first bad record.
#[test]
fn random_bytes_after_the_handshake_never_panic() {
    let (mut wire, mut stream) = open();
    let mut s = 0x7EC0_0001u32;
    let mut total = 0usize;
    for _ in 0..2_000 {
        let mut chunk: Vec<u8> = (0..xorshift(&mut s) % 300).map(|_| xorshift(&mut s) as u8).collect();
        if xorshift(&mut s).is_multiple_of(2) && chunk.len() > 5 {
            chunk[0] = APPLICATION_DATA;
        }
        total += chunk.len();
        wire.send_raw(&chunk);
        if let Ok(got) = stream.read(&mut wire) {
            assert!(got.len() <= total);
        }
        if stream.is_done() {
            break;
        }
    }
}
