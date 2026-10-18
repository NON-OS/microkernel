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

//! The relay's status line and head, hostile: a code that is not exactly
//! three digits in its place, another HTTP version, or a head that ends
//! past HEAD_MAX in a single read is refused.

use crate::fetch_desk::{Read, Refused, Reply};

fn done(b: &[u8], body: bool) -> Result<Read, Refused> {
    Reply::new(body).feed(b)
}

#[test]
fn a_status_line_is_exactly_its_shape() {
    assert_eq!(done(b"HTTP/1.0 200\r\n\r\n", false), Ok(Read::Done(200, Vec::new())));
    assert_eq!(done(b"HTTP/1.1 503 Busy\r\n\r\n", false), Ok(Read::Done(503, Vec::new())));
    for bad in [
        &b"HTTP/1.1 2000 OK\r\n\r\n"[..],
        b"HTTP/1.1x200 OK\r\n\r\n",
        b"HTTP/1.2 200 OK\r\n\r\n",
        b"HTTP/1.1 +20 OK\r\n\r\n",
        b"HTTP/1.1 20\r\n\r\n",
        b"HTTP/1.1  200 OK\r\n\r\n",
        b"\r\n\r\n",
    ] {
        assert_eq!(done(bad, false), Err(Refused), "{:?}", String::from_utf8_lossy(bad));
    }
}

#[test]
fn a_head_ending_past_its_bound_in_one_read_is_refused() {
    let long = [&b"HTTP/1.1 200 OK\r\nX: "[..], &[b'x'; 8 * 1024], b"\r\n\r\n"].concat();
    assert_eq!(done(&long, true), Err(Refused));
    let mut r = Reply::new(true);
    assert_eq!(r.feed(&long[..4096]), Ok(Read::More));
    assert_eq!(r.feed(&long[4096..]), Err(Refused), "the same head over two reads");
}
