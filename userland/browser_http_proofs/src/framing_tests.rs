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

//! One scanner frames, completes and parses: keep-alive offsets can never
//! land inside a header block or past a response parse refuses.

use crate::browser::http::response::{ends_at_close, frame_len, has_headers, is_complete, parse};
use crate::vectors::{read, response, PAGE};

#[test]
fn a_huge_second_content_length_frames_nothing() {
    /* The fuzz reproducer: Content-Length 47, then 2^64 - 1. It used to
    overflow body_at + n and frame inside the header block. */
    let raw = read("http_frame_len_cl_overflow.raw");
    assert_eq!(frame_len(&raw), None);
    assert!(parse(&raw).is_none());
    assert!(is_complete(&raw), "a head no byte can repair ends the fetch");
}

#[test]
fn conflicting_lengths_are_an_error_and_equal_ones_are_not() {
    let conflict = response(&["HTTP/1.1 200 OK", "Content-Length: 47", "Content-Length: 48"], PAGE);
    assert_eq!((frame_len(&conflict), parse(&conflict).is_none()), (None, true));
    let same = response(&["HTTP/1.1 200 OK", "Content-Length: 47, 47", "Content-Length: 47"], PAGE);
    assert_eq!(frame_len(&same), Some(same.len()));
    let signed = response(&["HTTP/1.1 200 OK", "Content-Length: +47"], PAGE);
    assert_eq!((frame_len(&signed), parse(&signed).is_none()), (None, true));
}

#[test]
fn a_204_ends_at_its_head_and_the_next_response_follows() {
    let next = response(&["HTTP/1.1 200 OK", "Content-Length: 47"], PAGE);
    let mut raw = response(&["HTTP/1.1 204 No Content", "Content-Length: 5"], b"");
    let head = raw.len();
    raw.extend_from_slice(&next);
    assert_eq!(frame_len(&raw), Some(head));
    assert!(is_complete(&raw[..head]) && parse(&raw).unwrap().body.is_empty());
    assert_eq!(parse(&raw[head..]).unwrap().body, PAGE);
}

#[test]
fn interim_responses_are_skipped_and_counted_in_the_frame() {
    let mut raw = response(&["HTTP/1.1 100 Continue"], b"");
    raw.extend(response(&["HTTP/1.1 103 Early Hints", "Link: </a.css>; rel=preload"], b""));
    assert!(!has_headers(&raw), "no final response yet");
    raw.extend(response(&["HTTP/1.1 200 OK", "Content-Length: 47"], PAGE));
    assert!(has_headers(&raw) && is_complete(&raw));
    assert_eq!(frame_len(&raw), Some(raw.len()));
    assert_eq!(parse(&raw).map(|r| (r.status, r.body)), Some((200, PAGE.to_vec())));
}

#[test]
fn obs_fold_obs_text_and_bare_lf_heads_parse() {
    let raw = b"HTTP/1.1 302 Found\nX-A: a\n\tb\nLocation: /caf\xe9\nContent-Length: 0\n\n";
    let r = parse(raw).unwrap();
    assert_eq!((r.status, r.location.as_deref()), (302, Some("/caf%E9")));
    assert_eq!(frame_len(raw), Some(raw.len()));
}

#[test]
fn only_a_response_with_no_framing_ends_at_the_close() {
    let open = response(&["HTTP/1.1 200 OK", "Content-Type: text/html"], PAGE);
    assert!(ends_at_close(&open) && !is_complete(&open), "it runs until the server closes");
    let sized = response(&["HTTP/1.1 200 OK", "Content-Length: 99"], PAGE);
    assert!(!ends_at_close(&sized), "it declared a length: closing short cuts it");
    let chunked = response(&["HTTP/1.1 200 OK", "Transfer-Encoding: chunked"], b"5\r\nhello\r\n");
    assert!(!ends_at_close(&chunked), "chunks end with their own last chunk");
    let empty = response(&["HTTP/1.1 304 Not Modified"], b"");
    assert!(!ends_at_close(&empty) && is_complete(&empty), "no content whatever it says");
    assert!(!ends_at_close(b"HTTP/1.1 200 OK\r\nConte"), "no whole head yet");
}
