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
//! HTTP/1.1 for a client, with no I/O of its own.

//! What each response head leads to.

use nonos_download::{decide, parse, parse_head, request, Decision, MAX_BYTES, MAX_REDIRECTS, TOO_LARGE};

fn head(text: &str) -> nonos_download::Head {
    parse_head(text.as_bytes()).expect("a head")
}

#[test]
fn a_cdn_redirect_is_followed_relative_or_whole() {
    let url = parse("https://example.org/music/song.mp3?x=1").unwrap();
    let whole = head("HTTP/1.1 302 Found\r\nLocation: https://cdn.example.net/a/b.mp3?sig=9\r\n\r\n");
    let Decision::Redirect(next) = decide(&whole, &url, 0, 0) else { panic!() };
    assert_eq!((next.host.as_str(), next.target.as_str()), ("cdn.example.net", "/a/b.mp3?sig=9"));
    let relative = head("HTTP/1.1 301 Moved\r\nlocation: other.mp3\r\n\r\n");
    let Decision::Redirect(next) = decide(&relative, &url, 0, 0) else { panic!() };
    assert_eq!((next.host.as_str(), next.target.as_str()), ("example.org", "/music/other.mp3"));
    let rooted = head("HTTP/1.1 308 P\r\nLocation: /x/y.mp3\r\n\r\n");
    let Decision::Redirect(next) = decide(&rooted, &url, 0, 0) else { panic!() };
    assert_eq!(next.target, "/x/y.mp3");
}

#[test]
fn redirects_stop_at_the_limit_and_never_go_to_plain_http() {
    let url = parse("https://example.org/a.mp3").unwrap();
    let h = head("HTTP/1.1 302 Found\r\nLocation: https://example.org/b.mp3\r\n\r\n");
    assert!(matches!(decide(&h, &url, 0, MAX_REDIRECTS), Decision::Refuse(_)));
    let down = head("HTTP/1.1 302 Found\r\nLocation: http://example.org/b.mp3\r\n\r\n");
    assert!(matches!(decide(&down, &url, 0, 0), Decision::Refuse(_)));
}

#[test]
fn a_file_over_200_mb_is_refused_before_any_byte_is_written() {
    let url = parse("https://example.org/a.mp3").unwrap();
    let big = head(&format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", MAX_BYTES + 1));
    assert_eq!(decide(&big, &url, 0, 0), Decision::Refuse(TOO_LARGE));
    let fits = head(&format!("HTTP/1.1 200 OK\r\nContent-Length: {MAX_BYTES}\r\n\r\n"));
    assert_eq!(decide(&fits, &url, 0, 0), Decision::Body { at: 0, total: Some(MAX_BYTES), chunked: false });
    assert_eq!(TOO_LARGE, "That file is larger than 200 MB, the most Music downloads.");
}

#[test]
fn a_resume_takes_the_rest_only_from_where_the_file_stopped() {
    let url = parse("https://example.org/a.mp3").unwrap();
    let rest = head("HTTP/1.1 206 Partial\r\nContent-Range: bytes 1000-4999/5000\r\n\r\n");
    assert_eq!(decide(&rest, &url, 1000, 0), Decision::Body { at: 1000, total: Some(5000), chunked: false });
    assert!(matches!(decide(&rest, &url, 999, 0), Decision::Refuse(_)), "the wrong place is not appended");
    // A server that ignores the range sends it all: the file starts over.
    let all = head("HTTP/1.1 200 OK\r\nContent-Length: 5000\r\n\r\n");
    assert_eq!(decide(&all, &url, 1000, 0), Decision::Body { at: 0, total: Some(5000), chunked: false });
    let past = head("HTTP/1.1 206 Partial\r\nContent-Range: bytes 1000-1/300000000\r\n\r\n");
    assert_eq!(decide(&past, &url, 1000, 0), Decision::Refuse(TOO_LARGE));
    let whole = head("HTTP/1.1 416 Range Not Satisfiable\r\n\r\n");
    assert_eq!(decide(&whole, &url, 5000, 0), Decision::Body { at: 5000, total: Some(5000), chunked: false });
}

#[test]
fn errors_say_what_happened() {
    let url = parse("https://example.org/a.mp3").unwrap();
    for (status, word) in [(404, "no file"), (403, "refused"), (429, "limiting"), (503, "failed")] {
        let h = head(&format!("HTTP/1.1 {status} X\r\n\r\n"));
        let Decision::Refuse(said) = decide(&h, &url, 0, 0) else { panic!("{status}") };
        assert!(said.contains(word), "{status}: {said}");
    }
}

#[test]
fn the_request_asks_for_the_file_as_it_is_and_the_rest_when_resuming() {
    let url = parse("https://cdn.example.net:8443/a/b.mp3?sig=9").unwrap();
    let first = request(&url, 0);
    assert!(first.starts_with("GET /a/b.mp3?sig=9 HTTP/1.1\r\nHost: cdn.example.net:8443\r\n"));
    assert!(first.contains("Accept-Encoding: identity\r\n") && !first.contains("Range:"));
    assert!(request(&url, 12345).contains("Range: bytes=12345-\r\n"));
    assert!(first.ends_with("\r\n\r\n"));
}

#[test]
fn chunked_and_types_are_read_from_the_head() {
    let h = head("HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip, Chunked\r\nContent-Type: Audio/MPEG\r\n\r\n");
    assert!(h.chunked);
    assert_eq!(h.content_type.as_deref(), Some("audio/mpeg"));
    assert!(parse_head(b"SSH-2.0-OpenSSH\r\n\r\n").is_none());
}
