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

//! What fetch says to the relay and how it reads the answer: a path under
//! the anchor's hex, a head and a body of exactly its length, nothing more.

use crate::fetch_desk::{get_request, put_request, verdict, Read, Refused, Reply};

#[test]
fn requests_name_the_anchor_and_carry_the_container() {
    let a = [0xAB; 32];
    let get = String::from_utf8(get_request("relay.example", &a)).expect("text");
    assert_eq!(
        get,
        format!(
            "GET /v1/proof/{} HTTP/1.1\r\nHost: relay.example\r\nConnection: close\r\n\r\n",
            "ab".repeat(32)
        )
    );
    let put = put_request("r", &a, b"NPRF..");
    assert!(put.starts_with(format!("PUT /v1/proof/{} HTTP/1.1\r\n", "ab".repeat(32)).as_bytes()));
    assert!(put.ends_with(b"Content-Length: 6\r\nConnection: close\r\n\r\nNPRF.."));
}

#[test]
fn a_reply_is_a_head_and_exactly_its_length() {
    let mut r = Reply::new(true);
    assert_eq!(r.feed(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n"), Ok(Read::More));
    assert_eq!(r.feed(b"\r\nab"), Ok(Read::More));
    assert_eq!(r.feed(b"cd"), Ok(Read::Done(200, b"abcd".to_vec())));
    let done = |b: &[u8], body: bool| Reply::new(body).feed(b);
    assert_eq!(done(b"HTTP/1.1 201 Created\r\n\r\n", false), Ok(Read::Done(201, Vec::new())));
    assert_eq!(
        done(b"HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\n", true),
        Ok(Read::Done(404, Vec::new()))
    );
    for bad in [
        &b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n"[..],
        b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\nContent-Length: 1\r\n\r\nx",
        b"HTTP/1.1 200 OK\r\n\r\n",
        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nabc",
        b"HTTP/1.1 200 OK\r\nContent-Length: 1048577\r\n\r\n",
        b"SSH-2.0-OpenSSH\r\n\r\n",
    ] {
        assert_eq!(done(bad, true), Err(Refused), "{:?}", String::from_utf8_lossy(bad));
    }
    assert_eq!(done(&[b'x'; 8 * 1024 + 1], true), Err(Refused), "a head past HEAD_MAX");
}

#[test]
fn the_relays_codes_end_an_op_or_ask_for_another_connection() {
    assert_eq!(verdict(true, 201, vec![1]), Ok(Vec::new()));
    assert_eq!(verdict(false, 200, vec![1]), Ok(vec![1]));
    assert_eq!(verdict(false, 404, Vec::new()), Err(Some(-2)));
    assert_eq!(verdict(true, 503, Vec::new()), Err(None));
    assert_eq!(verdict(true, 404, Vec::new()), Err(Some(-1206)));
    assert_eq!(verdict(false, 302, Vec::new()), Err(Some(-1206)));
}
