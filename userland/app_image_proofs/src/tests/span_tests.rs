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

//! A chunk's reply is taken only when it is exactly the range asked.

use crate::install::image::span::{chunk, request};

fn reply(status: &str, headers: &str, body: &[u8]) -> Vec<u8> {
    let mut r = format!("HTTP/1.1 {status}\r\n{headers}Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    r.extend_from_slice(body);
    r
}

#[test]
fn the_request_names_the_range_inclusive() {
    let r = request("/linux/ab.image", "mirror.example", &(4096..8192));
    assert!(r.starts_with("GET /linux/ab.image HTTP/1.1\r\nHost: mirror.example\r\n"));
    assert!(r.contains("\r\nRange: bytes=4096-8191\r\n"));
    assert!(r.ends_with("\r\n\r\n"));
}

#[test]
fn a_206_of_the_range_asked_is_the_chunk() {
    let body = vec![9u8; 100];
    let ok = reply("206 Partial Content", "Content-Range: bytes 100-199/1000\r\n", &body);
    assert_eq!(chunk(ok, &(100..200), 1000), Some(body.clone()));
    for (cr, total) in [("bytes 101-200/1000", 1000), ("bytes 100-199/999", 1000), ("bytes 100-199/*", 1000)] {
        let r = reply("206 Partial Content", &format!("Content-Range: {cr}\r\n"), &body);
        assert_eq!(chunk(r, &(100..200), total), None, "{cr}");
    }
}

#[test]
fn a_200_is_taken_only_for_the_whole_object() {
    let body = vec![1u8; 50];
    assert_eq!(chunk(reply("200 OK", "", &body), &(0..50), 50), Some(body.clone()));
    assert_eq!(chunk(reply("200 OK", "", &body), &(0..50), 100), None, "a mirror that ignored Range");
    assert_eq!(chunk(reply("404 Not Found", "", &body), &(0..50), 50), None);
}

#[test]
fn a_short_or_long_body_is_refused() {
    let mut short = reply("200 OK", "", &[1u8; 50]);
    short.truncate(short.len() - 1);
    assert_eq!(chunk(short, &(0..50), 50), None);
    let mut long = reply("200 OK", "", &[1u8; 50]);
    long.push(0);
    assert_eq!(chunk(long, &(0..50), 50), None);
    assert_eq!(chunk(b"garbage".to_vec(), &(0..50), 50), None);
}
