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

//! What reply ends a lookup: only one from the server, to this query.

use crate::protocol::E_SERVFAIL;
use crate::upstream::{answer, fresh, queries, resolve_a, script, Datagram, SERVER};

const IP: [u8; 4] = [192, 0, 2, 80];

/// Builds, from the query's bytes, a datagram that is not a reply to it.
type NotAReply = fn(&[u8]) -> Vec<u8>;

/*
 * Before the real answer, the server's address and port deliver something
 * that is not a reply to this query. It used to be parsed first and a parse
 * failure taken as the server's verdict, so one malformed or echoed packet
 * with a guessable source ended the lookup as SERVFAIL.
 */
#[test]
fn what_is_not_a_reply_to_this_query_is_ignored() {
    let not_replies: [NotAReply; 4] = [
        |_| vec![0xde, 0xad],
        |q| q[..5].to_vec(),
        |q| q.to_vec(),
        |q| {
            let mut m = q.to_vec();
            m[2] = 0x81;
            let last = m.len() - 5;
            m[last] ^= 0x20 ^ 0x01;
            m.push(0xC0);
            m
        },
    ];
    for (i, bad) in not_replies.into_iter().enumerate() {
        let _g = fresh();
        script(move |q| vec![Datagram::from_server(bad(&q.bytes)), Datagram::from_server(answer(q, IP, 60))]);
        assert_eq!(resolve_a("example.com"), (0, Some(IP)), "case {i}");
    }
}

#[test]
fn a_reply_from_anyone_but_the_server_is_ignored() {
    let _g = fresh();
    script(|q| {
        let forged = answer(q, [6, 6, 6, 6], 60);
        vec![
            Datagram { src: [6, 6, 6, 6], sport: 53, payload: forged.clone() },
            Datagram { src: SERVER, sport: 5353, payload: forged },
            Datagram::from_server(answer(q, IP, 60)),
        ]
    });
    assert_eq!(resolve_a("example.com"), (0, Some(IP)));
}

#[test]
fn a_reply_with_another_id_is_ignored() {
    let _g = fresh();
    script(|q| {
        let mut stale = answer(q, [6, 6, 6, 6], 60);
        stale[0] ^= 0xFF;
        vec![Datagram::from_server(stale), Datagram::from_server(answer(q, IP, 60))]
    });
    assert_eq!(resolve_a("example.com"), (0, Some(IP)));
}

#[test]
fn silence_ends_in_servfail_after_the_deadline() {
    let _g = fresh();
    script(|_| Vec::new());
    assert_eq!(resolve_a("example.com"), (E_SERVFAIL, None));
    assert!(queries() >= 2, "the query was sent again while waiting");
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/// Damaged and random replies from the server never panic a lookup, and an
/// address that comes back is one the server put in a reply to this query.
#[test]
fn damaged_replies_never_panic_a_lookup() {
    let _g = fresh();
    let mut seed = 0xD0D0_0001u32;
    for round in 0..3_000u32 {
        crate::state::CACHE.lock().tick(u64::MAX);
        let s0 = xorshift(&mut seed);
        script(move |q| {
            let mut s = s0 | 1;
            let mut m = answer(q, IP, 60);
            for _ in 0..1 + xorshift(&mut s) % 4 {
                let at = xorshift(&mut s) as usize % m.len();
                m[at] = xorshift(&mut s) as u8;
            }
            let noise: Vec<u8> = (0..xorshift(&mut s) % 80).map(|_| xorshift(&mut s) as u8).collect();
            vec![Datagram::from_server(noise), Datagram::from_server(m)]
        });
        let (errno, ip) = resolve_a(&format!("host{round}.example"));
        if errno == 0 {
            assert!(ip.is_some());
        }
    }
}
