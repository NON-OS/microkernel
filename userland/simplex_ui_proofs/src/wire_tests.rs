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

//! The WebSocket handshake and frames. The SERVE_ONE frames are the serve-wire crate's
//! own (nonos_serve_wire), held by its tests.

use crate::link::ws_frame::text;
use crate::link::ws_hello::{answer, request, Answer};
use crate::link::ws_read::{take, Got};

#[test]
fn the_upgrade_asks_for_websocket_with_the_rfc_key() {
    let r = request(b"the sample nonce");
    assert!(r.starts_with("GET / HTTP/1.1\r\n"));
    assert!(r.contains("Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"));
    assert!(r.ends_with("\r\n\r\n"));
}

#[test]
fn each_header_is_a_line_of_its_own_and_none_folds_into_another() {
    // A line that begins with a space or a tab continues the header before
    // it (RFC 9112 obs-fold): the core then finds no key and closes.
    let r = request(b"the sample nonce");
    let head = r.strip_suffix("\r\n\r\n").expect("the head ends with a blank line");
    let mut lines = head.split("\r\n");
    assert_eq!(lines.next(), Some("GET / HTTP/1.1"));
    let headers: Vec<(&str, &str)> = lines
        .map(|l| {
            assert!(!l.starts_with([' ', '\t']), "folded: {l:?}");
            assert!(!l.contains('\n') && !l.contains('\r'), "a bare line end in {l:?}");
            l.split_once(": ").expect("a name and a value")
        })
        .collect();
    let names: Vec<&str> = headers.iter().map(|h| h.0).collect();
    let want = ["Host", "Upgrade", "Connection", "Sec-WebSocket-Key", "Sec-WebSocket-Version"];
    assert_eq!(names, want);
    assert!(headers.iter().all(|(n, v)| n.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        && !v.starts_with(' ')
        && !v.ends_with(' ')));
    assert_eq!(headers[2].1, "Upgrade", "Connection holds Upgrade alone");
    assert_eq!(headers[3].1, "dGhlIHNhbXBsZSBub25jZQ==");
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
