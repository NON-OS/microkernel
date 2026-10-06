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

//! Chunked framing: one walker for completion, frame end and decoding.

use crate::browser::http::chunked::{complete, decode, decode_partial, frame_end};

#[test]
fn whitespace_before_an_extension_and_after_a_size_is_accepted() {
    assert_eq!(
        decode(b"3 ;name=val\r\nabc\r\n2\t\r\nde\r\n0\r\n\r\n").as_deref(),
        Some(&b"abcde"[..])
    );
    assert_eq!(decode(b"3;q=\"a;b\"\r\nabc\r\n0\r\n\r\n").as_deref(), Some(&b"abc"[..]));
}

#[test]
fn bare_lf_lines_and_trailers_frame() {
    let body = b"3\nabc\n0\nX-T: 1\n\nnext";
    assert_eq!(frame_end(body), Some(body.len() - 4));
    assert!(complete(&body[..body.len() - 4]) && !complete(&body[..body.len() - 5]));
}

#[test]
fn a_cut_body_keeps_its_chunks_and_a_bad_one_is_refused() {
    assert_eq!(decode_partial(b"7\r\n<!docty\r\n7\r\n><t"), Some((b"<!docty><t".to_vec(), false)));
    assert_eq!(decode_partial(b"3\r\nabc\r\n0\r\n"), Some((b"abc".to_vec(), false)));
    assert_eq!(decode_partial(b"3\r\nabc\r\n0\r\n\r\n"), Some((b"abc".to_vec(), true)));
    assert_eq!(decode_partial(b"5\r\nabc\r\n0\r\n\r\n"), None, "data runs into the next line");
    assert_eq!(decode_partial(b"zz\r\nabc"), None);
    assert!(decode(b"3\r\nabc\r\n0\r\n").is_none(), "strict decoding wants the end");
}

#[test]
fn a_size_past_a_signed_64_bit_count_is_malformed() {
    assert_eq!(decode_partial(b"ffffffffffffffff\r\nabc\r\n0\r\n\r\n"), None);
    assert_eq!(decode_partial(b"1ffffffffffffffff\r\nabc"), None);
    assert_eq!(decode_partial(b"7fffffffffffffff\r\nabc"), Some((b"abc".to_vec(), false)));
    assert_eq!(frame_end(b"7fffffffffffffff\r\nabc"), None);
}
