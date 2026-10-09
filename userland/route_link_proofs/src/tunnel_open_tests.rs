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

//! Opening a stream through a proxy: a reset first, the greeting and the
//! CONNECT under numbers that start at one, a reply waited for however it
//! arrives, and a refusal that names the network and ends the conversation.

use crate::bounds::{OPEN_MS, RESET_WAIT_MS, SEND_WAIT_MS};
use crate::fake::{FakeProxy, Shared};
use crate::refusal::{Proxy, BAD_HOST};
use crate::tunnel::Tunnel;

#[test]
fn a_stream_opens_with_a_reset_then_numbered_socks() {
    let Ok(t) = Tunnel::open(FakeProxy::nym(), Proxy::Nym, "example.com", 443) else {
        panic!("did not open")
    };
    let fake = &t.carrier;
    assert_eq!(fake.frames[0], [1], "a reset comes first");
    let n = fake.numbered();
    assert_eq!(n[0], (1, vec![5, 1, 0]));
    assert_eq!(n[1].0, 2);
    assert_eq!(&n[1].1[..5], &[5, 1, 0, 3, 11]);
    assert_eq!(fake.host, b"example.com");
    assert_eq!(fake.port, 443);
    assert!(t.pending.is_empty() && !t.closed && t.seq == 3);
}

#[test]
fn dropping_a_stream_resets_it_at_the_proxy() {
    let fake = Shared::new(FakeProxy::nym());
    let t = Tunnel::open(fake.clone(), Proxy::Nym, "example.com", 443);
    assert!(t.is_ok());
    assert_eq!(fake.0.borrow().resets, 1);
    drop(t);
    let after = fake.0.borrow();
    assert_eq!(after.resets, 2);
    assert_eq!(after.frames.last().map(|f| f.as_slice()), Some(&[1u8][..]));
}

#[test]
fn net_anon_answers_the_connect_once_its_exit_connects() {
    let Ok(t) = Tunnel::open(FakeProxy::anon(6), Proxy::Anyone, "example.com", 443) else {
        panic!("did not open")
    };
    let n = t.carrier.numbered();
    /* Greeting, CONNECT, then empty polls under numbers that move on. */
    let polls: Vec<_> = n.iter().skip(2).collect();
    assert_eq!(polls.len(), 6);
    for (i, (seq, body)) in polls.iter().enumerate() {
        assert_eq!(*seq, 3 + i as u32);
        assert!(body.is_empty());
    }
}

#[test]
fn a_reply_split_across_answers_is_waited_for() {
    let mut fake = FakeProxy::nym();
    fake.split_reply = true;
    let Ok(t) = Tunnel::open(fake, Proxy::Nym, "example.com", 443) else { panic!("did not open") };
    assert!(t.pending.is_empty(), "the reply was taken whole and nothing else");
}

#[test]
fn a_refused_connect_names_the_network_and_its_cause_and_is_reset() {
    let cases = [
        (
            2,
            "the Nym exit's rules refuse that destination",
            "the Anyone exit's policy refuses that destination",
        ),
        (
            4,
            "the Nym mixnet has no exit for that destination",
            "the Anyone exit could not resolve the host",
        ),
        (
            5,
            "the Nym gateway refused the request",
            "the host refused the connection from the Anyone exit",
        ),
    ];
    for (rep, nym, anyone) in cases {
        let fake = Shared::new(FakeProxy { rep, ..FakeProxy::nym() });
        assert_eq!(Tunnel::open(fake.clone(), Proxy::Nym, "h.example", 443).err(), Some(nym));
        assert_eq!(fake.0.borrow().resets, 2, "reset on open and on the way out");
        let fake = FakeProxy { rep, ..FakeProxy::anon(2) };
        assert_eq!(Tunnel::open(fake, Proxy::Anyone, "h.example", 443).err(), Some(anyone));
    }
}

#[test]
fn a_full_proxy_says_so() {
    let fake = FakeProxy { method: 0xFF, ..FakeProxy::nym() };
    let why = Tunnel::open(fake, Proxy::Nym, "h.example", 443).err();
    assert_eq!(why, Some(Proxy::Nym.full()));
}

#[test]
fn a_network_not_connected_yet_is_asked_again_from_a_reset_then_refused_in_time() {
    let fake = Shared::new(FakeProxy { rep: 3, ..FakeProxy::nym() });
    let start = fake.0.borrow().now;
    let why = Tunnel::open(fake.clone(), Proxy::Nym, "h.example", 443).err();
    assert_eq!(why, Some("the Nym mixnet is not connected yet"));
    let f = fake.0.borrow();
    assert!(f.resets > 3, "asked again, each time from a reset: {}", f.resets);
    let spent = f.now - start;
    assert!(spent >= OPEN_MS, "gave the network its time: {spent}");
    assert!(spent <= OPEN_MS + 5_000, "and no more: {spent}");
}

#[test]
fn a_dead_proxy_is_silent_within_a_bound() {
    let fake = Shared::new(FakeProxy { dead: true, ..FakeProxy::nym() });
    let start = fake.0.borrow().now;
    let why = Tunnel::open(fake.clone(), Proxy::Anyone, "h.example", 443).err();
    assert_eq!(why, Some("net.anon stopped answering"));
    /* The reset going in, three tries at the greeting, the reset going out. */
    let spent = fake.0.borrow().now - start;
    let bound = 2 * RESET_WAIT_MS as i64 + 3 * SEND_WAIT_MS as i64 + 1_000;
    assert!(spent <= bound, "{spent} > {bound}");
}

#[test]
fn a_host_socks_cannot_carry_sends_nothing() {
    let fake = Shared::new(FakeProxy::nym());
    let why = Tunnel::open(fake.clone(), Proxy::Nym, &"a".repeat(256), 443).err();
    assert_eq!(why, Some(BAD_HOST));
    assert_eq!(fake.0.borrow().calls, 0);
}
