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

//! The agent's HTTP reads a POST of exactly BODY_LEN and nothing else; its
//! answer carries the sealed bytes.

use std::io::Cursor;

use super::super::body::BODY_LEN;
use super::super::http::{read_request, write_answer};

fn post(len: usize, body: &[u8]) -> Vec<u8> {
    let head = format!("POST /e HTTP/1.1\r\nHost: a\r\nContent-Length: {len}\r\n\r\n");
    [head.as_bytes(), body].concat()
}

#[test]
fn only_a_post_of_body_len_is_read() {
    let body = vec![7u8; BODY_LEN];
    assert_eq!(read_request(&mut Cursor::new(post(BODY_LEN, &body))), Some(body.clone()));
    assert_eq!(read_request(&mut Cursor::new(post(BODY_LEN + 1, &body))), None);
    let get = post(BODY_LEN, &body).iter().skip(4).copied().collect::<Vec<u8>>();
    assert_eq!(read_request(&mut Cursor::new([b"GET ".as_slice(), &get].concat())), None);
    assert_eq!(read_request(&mut Cursor::new(vec![b'a'; 5000])), None, "no head end");
    assert_eq!(read_request(&mut Cursor::new(post(BODY_LEN, &body[1..]))), None, "short body");
}

#[test]
fn an_answer_carries_its_status_and_bytes() {
    let mut out = Vec::new();
    write_answer(&mut out, 200, &[1, 2, 3]).unwrap();
    let text = String::from_utf8_lossy(&out);
    assert!(text.starts_with("HTTP/1.1 200 OK\r\n") && text.contains("Content-Length: 3\r\n"));
    assert!(out.ends_with(&[1, 2, 3]));
}
