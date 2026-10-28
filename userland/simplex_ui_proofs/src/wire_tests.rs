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

//! The NSO1 head and the WebSocket handshake and frames.

use crate::link::nso1::{message, parse, Head, FRAME_MAX, HEAD, READ, WRITE};
use crate::link::ws_frame::text;
use crate::link::ws_hello::{answer, request, Answer};
use crate::link::ws_read::{take, Got};

#[test]
fn an_nso1_message_reads_back_as_written() {
    let m = message(WRITE, 7, b"hello");
    assert_eq!(&m[..4], b"NSO1");
    assert_eq!(m.len(), HEAD + 5);
    let (head, body) = parse(&m).unwrap();
    assert_eq!(head, Head { op: WRITE, conn: 7, len: 5 });
    assert_eq!(body, b"hello");
}

#[test]
fn an_nso1_reply_short_of_its_length_or_not_nso1_is_none() {
    let m = message(READ, 1, b"abc");
    assert!(parse(&m[..m.len() - 1]).is_none());
    assert!(parse(b"NSO2\x03\x00\x01\x00\x00\x00\x00\x00").is_none());
    assert_eq!(message(WRITE, 1, &vec![0; FRAME_MAX + 9]).len(), HEAD + FRAME_MAX);
}

#[test]
fn the_upgrade_asks_for_websocket_with_the_rfc_key() {
    let r = request(b"the sample nonce");
    assert!(r.starts_with("GET / HTTP/1.1\r\n"));
    assert!(r.contains("Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"));
    assert!(r.ends_with("\r\n\r\n"));
}

#[test]
fn only_a_whole_101_upgrades() {
    assert!(matches!(answer(b"HTTP/1.1 101 Switching"), Answer::More));
    let ok = b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\n\r\n\x81";
    assert!(matches!(answer(ok), Answer::Upgraded(n) if n == ok.len() - 1));
    assert!(matches!(answer(b"HTTP/1.1 400 Bad\r\n\r\n"), Answer::Refused));
}

#[test]
fn a_masked_text_frame_unmasks_to_its_text() {
    let mut f = text("{\"cmd\":\"/_get chats 1\"}", [1, 2, 3, 4]);
    assert_eq!(f[0], 0x81);
    assert_eq!(f[1] & 0x80, 0x80);
    assert_eq!(take(&mut f), Some(Got::Text("{\"cmd\":\"/_get chats 1\"}".into())));
    assert!(f.is_empty());
}

#[test]
fn a_frame_is_taken_only_once_whole_and_long_lengths_read() {
    let long = "x".repeat(70_000);
    let mut f = text(&long, [9, 9, 9, 9]);
    let mut part = f[..1000].to_vec();
    assert_eq!(take(&mut part), None);
    assert_eq!(f[1] & 0x7f, 127);
    assert_eq!(take(&mut f), Some(Got::Text(long)));
}
