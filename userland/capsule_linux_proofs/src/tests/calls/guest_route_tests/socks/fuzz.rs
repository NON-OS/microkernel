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

//! Whatever a guest writes, in whatever pieces: no panic, nothing taken
//! past what was written or past the bound, and a CONNECT only ever names
//! a name the endpoint would carry or an address.

use super::super::super::mutation::damage;
use super::{name, next, v4a_request, v5_request};
use crate::linux::net::routed::socks::kinds::{name_ok, REQUEST_MAX};
use crate::linux::net::routed::socks::{v4, v5, Heard, Talk, To};

/// Hear `bytes` in random pieces; what was heard, checked as it goes.
fn talk(s: &mut u64, bytes: &[u8]) -> Heard {
    let (mut t, mut said, mut at) = (Talk::default(), Vec::new(), 0);
    loop {
        let piece = &bytes[at..(at + 1 + (next(s) % 64) as usize).min(bytes.len())];
        let (took, heard) = t.hear(piece, &mut said);
        assert!(took <= piece.len() && took <= REQUEST_MAX);
        at += took;
        if heard != Heard::More || took == 0 || at >= bytes.len() {
            if let Heard::Connect(_, c) = &heard {
                assert!(matches!(&c.to, To::V4(_)) || matches!(&c.to, To::Name(n) if name_ok(n)));
            }
            return heard;
        }
    }
}

#[test]
fn random_bytes_never_panic_or_overrun() {
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..20_000 {
        let len = (next(&mut s) % 700) as usize;
        let mut bytes: Vec<u8> = (0..len).map(|_| next(&mut s) as u8).collect();
        if let Some(first) = bytes.first_mut() {
            *first = [4, 5, *first][(next(&mut s) % 3) as usize];
        }
        let _ = (v4::request(&bytes), v5::greeting(&bytes), v5::request(&bytes));
        let _ = talk(&mut s, &bytes);
    }
}

#[test]
fn damaged_requests_never_panic_and_never_change_the_name_unseen() {
    let mut s = 0x2545_F491_4F6C_DD1Du64;
    let good = [
        v4a_request(b"node.example.org", 18089),
        [&[5u8, 1, 0][..], &v5_request(1, &name("node.example.org", 18089).to, 18089)].concat(),
    ];
    for _ in 0..20_000 {
        for g in &good {
            let mut v = g.clone();
            damage(&mut s, &mut v);
            if let Heard::Connect(_, c) = talk(&mut s, &v) {
                assert!(c.port != 0);
            }
        }
    }
}
