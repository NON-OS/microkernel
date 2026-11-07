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

//! RFC 1929: a username and password, as SimpleX sends to isolate its
//! streams, read whole wherever it is split, answered, let go, and never
//! a way past the request that follows.

use super::{name, sign_in_bytes, v5_request};
use crate::linux::net::routed::socks::kinds::REQUEST_MAX;
use crate::linux::net::routed::socks::parsed::Parsed;
use crate::linux::net::routed::socks::sign_in::credentials;
use crate::linux::net::routed::socks::{Heard, Talk, Ver};

/// Hear `bytes` cut at `cut`: taken in all, said, heard.
fn split(bytes: &[u8], cut: usize) -> (usize, Vec<u8>, Heard) {
    let (mut t, mut said) = (Talk::default(), Vec::new());
    let (a, first) = t.hear(&bytes[..cut], &mut said);
    if first != Heard::More {
        return (a, said, first);
    }
    let (b, heard) = t.hear(&bytes[cut..], &mut said);
    (a + b, said, heard)
}

#[test]
fn a_pair_of_any_lengths_is_taken_whole_and_every_cut_is_more_to_come() {
    let long = vec![b'u'; 255];
    for (user, password) in [(&b"a1b2c3"[..], &b""[..]), (b"", b""), (&long, &long), (b"u", b"p")] {
        let pair = sign_in_bytes(user, password);
        assert_eq!(credentials(&pair), Parsed::Done((), pair.len()));
        for cut in 0..pair.len() {
            assert_eq!(credentials(&pair[..cut]), Parsed::Need, "cut at {cut}");
        }
        let more = [&pair[..], &[5, 1, 0]].concat();
        assert_eq!(credentials(&more), Parsed::Done((), pair.len()), "what follows is not taken");
    }
    assert_eq!(sign_in_bytes(&long, &long).len(), REQUEST_MAX, "the longest message held");
    for ver in (0u8..=255).filter(|&v| v != 1) {
        assert_eq!(credentials(&[ver, 1, b'u', 0]), Parsed::Bad);
        assert_eq!(credentials(&[ver]), Parsed::Bad);
    }
}

#[test]
fn simplex_s_greeting_pair_and_connect_are_heard_wherever_they_are_split() {
    let onion = "smp4.simplex.example.onion";
    let req = [
        &[5u8, 1, 2][..],
        &sign_in_bytes(b"c2Vzc2lvbi0x", b""),
        &v5_request(1, &name(onion, 5223).to, 5223),
    ]
    .concat();
    for cut in 0..req.len() {
        let (took, said, heard) = split(&req, cut);
        assert_eq!(took, req.len(), "cut at {cut}");
        assert_eq!(said, [5, 2, 1, 0], "the method chosen, then the pair taken");
        assert_eq!(heard, Heard::Connect(Ver::Five, name(onion, 5223)));
    }
}

#[test]
fn the_longest_greeting_and_pair_still_pass_and_nothing_past_the_request_is_taken() {
    let long = vec![b'p'; 255];
    let greeting = [&[5u8, 255][..], &[2u8; 255]].concat();
    let req = v5_request(1, &name("n.example", 443).to, 443);
    let all = [&greeting[..], &sign_in_bytes(&long, &long), &req, b"GET /"].concat();
    let (mut t, mut said, mut at) = (Talk::default(), Vec::new(), 0);
    let heard = loop {
        let (took, heard) = t.hear(&all[at..], &mut said);
        assert!(took > 0 && took <= REQUEST_MAX);
        at += took;
        if heard != Heard::More {
            break heard;
        }
    };
    assert_eq!(heard, Heard::Connect(Ver::Five, name("n.example", 443)));
    assert_eq!(at, all.len() - 5, "the stream's own bytes are left to it");
    assert_eq!(said, [5, 2, 1, 0]);
}

#[test]
fn a_pair_in_another_version_is_refused_and_closed_and_no_request_is_heard() {
    let bytes = [&[5u8, 1, 2][..], &[2, 1, b'u', 0], &v5_request(1, &name("n.example", 80).to, 80)]
        .concat();
    let (mut t, mut said) = (Talk::default(), Vec::new());
    assert_eq!(t.hear(&bytes, &mut said).1, Heard::Closed);
    assert_eq!(said, [5, 2, 1, 1], "the method, then the refusal");
}

#[test]
fn a_connect_after_no_authentication_is_never_asked_for_a_pair() {
    let bytes = [&[5u8, 2, 0, 2][..], &v5_request(1, &name("n.example", 80).to, 80)].concat();
    let (mut t, mut said) = (Talk::default(), Vec::new());
    assert_eq!(
        t.hear(&bytes, &mut said),
        (bytes.len(), Heard::Connect(Ver::Five, name("n.example", 80)))
    );
    assert_eq!(said, [5, 0], "no authentication chosen, as before");
}
