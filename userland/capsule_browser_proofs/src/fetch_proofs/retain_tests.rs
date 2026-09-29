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

//! Only a response framed exactly, that did not ask to close, is kept.

use super::fetch_fixtures::url_of;
use crate::browser::fetch::pool::retain;
use crate::browser::fetch::types::{Fetch, Phase};

fn done(raw: &[u8]) -> Fetch {
    let mut f = Fetch::new(url_of("http://10.0.2.2/"), 11, Phase::Done, 0);
    f.keep = true;
    f.buf = raw.to_vec();
    f
}

fn kept(raw: &[u8]) -> bool {
    retain(&mut done(raw), raw, 0).is_some()
}

#[test]
fn which_responses_leave_their_connection() {
    assert!(kept(b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\nx"));
    assert!(kept(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n"));
    assert!(kept(b"HTTP/1.0 200 OK\r\nConnection: keep-alive\r\nContent-Length: 0\r\n\r\n"));
    assert!(!kept(b"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 1\r\n\r\nx"));
    assert!(!kept(b"HTTP/1.0 200 OK\r\nContent-Length: 1\r\n\r\nx"), "1.0 closes");
    assert!(!kept(b"HTTP/1.1 200 OK\r\n\r\nno length"), "ended by quiet, not framing");
    assert!(!kept(b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\nxy"), "bytes past the end");
}

#[test]
fn a_post_or_an_unkept_request_is_not_kept() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\nx";
    let mut post = done(raw);
    post.post = Some(String::from("a=1"));
    assert!(retain(&mut post, raw, 0).is_none());
    let mut closing = done(raw);
    closing.keep = false;
    assert!(retain(&mut closing, raw, 0).is_none());
}
