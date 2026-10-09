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

//! Interim responses (RFC 9110 15.2) come before the final one and are not it.

use nonos_http::parse_response;

/*
 * A 103 Early Hints (sent by large CDNs to any client) or a 100 Continue was
 * taken as the response: status 103, and the real response, head and all,
 * handed on as its body.
 */
#[test]
fn early_hints_and_continue_are_passed_over() {
    let raw = b"HTTP/1.1 103 Early Hints\r\nLink: </s.css>; rel=preload\r\n\r\n\
HTTP/1.1 100 Continue\r\n\r\n\
HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok";
    let r = parse_response(raw).expect("parse");
    assert_eq!(r.status, 200);
    assert_eq!(r.body, b"ok");
    assert_eq!(r.header("link"), None, "the hints are not the final response's fields");
}

#[test]
fn switching_protocols_is_final() {
    let r = parse_response(b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: x\r\n\r\nnot http").expect("parse");
    assert_eq!(r.status, 101);
    assert!(r.body.is_empty(), "what follows is another protocol, not a body");
}

#[test]
fn an_endless_run_of_interim_responses_is_refused() {
    let mut raw = Vec::new();
    for _ in 0..64 {
        raw.extend_from_slice(b"HTTP/1.1 100 Continue\r\n\r\n");
    }
    raw.extend_from_slice(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
    assert!(parse_response(&raw).is_err());
}
