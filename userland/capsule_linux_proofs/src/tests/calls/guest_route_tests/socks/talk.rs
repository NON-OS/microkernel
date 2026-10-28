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

//! A conversation, write by write: what it takes, what it says, and when
//! it has its CONNECT. Bytes past a whole request are never taken.

use super::{name, v4a_request, v5_request};
use crate::linux::net::routed::socks::{Heard, Talk, Ver};

/// Every write of `bytes` split at `cut`: what was taken in all, said, heard.
fn split(bytes: &[u8], cut: usize) -> (usize, Vec<u8>, Heard) {
    let (mut t, mut said) = (Talk::default(), Vec::new());
    let (a, first) = t.hear(&bytes[..cut], &mut said);
    assert_eq!(a, cut, "a part that ends no request is taken whole");
    if first != Heard::More {
        return (a, said, first);
    }
    let (b, heard) = t.hear(&bytes[cut..], &mut said);
    (a + b, said, heard)
}

#[test]
fn socks5_answers_the_greeting_then_hears_the_connect_wherever_it_is_split() {
    let req = [&[5u8, 1, 0][..], &v5_request(1, &name("n.example", 18089).to, 18089)].concat();
    for cut in 0..req.len() {
        let (took, said, heard) = split(&req, cut);
        assert_eq!(took, req.len(), "cut at {cut}");
        assert_eq!(said, [5, 0], "the method reply, and nothing else yet");
        assert_eq!(heard, Heard::Connect(Ver::Five, name("n.example", 18089)));
    }
}

#[test]
fn socks4a_is_heard_wherever_it_is_split() {
    let req = v4a_request(b"node.example.org", 18089);
    for cut in 0..req.len() {
        let (took, said, heard) = split(&req, cut);
        assert_eq!((took, said.len()), (req.len(), 0), "cut at {cut}");
        assert_eq!(heard, Heard::Connect(Ver::Four, name("node.example.org", 18089)));
    }
}

#[test]
fn bytes_past_the_request_are_left_for_the_stream() {
    let req = v4a_request(b"n.example", 80);
    let data = [&req[..], b"GET / HTTP/1.1\r\n\r\n"].concat();
    let (mut t, mut said) = (Talk::default(), Vec::new());
    assert_eq!(t.hear(&data[..3], &mut said).1, Heard::More);
    let (took, heard) = t.hear(&data[3..], &mut said);
    assert_eq!(took, req.len() - 3, "only the rest of the request is taken");
    assert_eq!(heard, Heard::Connect(Ver::Four, name("n.example", 80)));
}
