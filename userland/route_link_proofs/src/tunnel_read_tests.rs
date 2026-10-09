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

//! Carrying bytes once open: a reply that comes back over many polls, in
//! pieces and late, read whole and in order, and a close that ends the
//! reading without another word to the proxy.

use crate::carrier::Carrier;
use crate::fake::FakeProxy;
use crate::refusal::{Proxy, FINISHED};
use crate::tunnel::Tunnel;

const REQUEST: &[u8] = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
const PARTS: [&[u8]; 4] =
    [b"HTTP/1.1 200 OK\r\n", b"Content-Length: 11\r\n", b"\r\n", b"hello world"];

fn whole() -> Vec<u8> {
    PARTS.concat()
}

fn far_end(fake: FakeProxy) -> FakeProxy {
    let mut fake =
        fake.deliver(4, PARTS[0]).deliver(3, PARTS[1]).deliver(0, PARTS[2]).deliver(5, PARTS[3]);
    fake.close_after = true;
    fake
}

/* Read with a small buffer until the stream says it has ended. */
fn read_all<C: Carrier>(t: &mut Tunnel<C>) -> Vec<u8> {
    let mut got = Vec::new();
    let mut buf = [0u8; 7];
    for _ in 0..10_000 {
        match t.read_wait(&mut buf, 60_000) {
            Ok(0) => break,
            Ok(n) => got.extend_from_slice(&buf[..n]),
            Err(why) => panic!("{why}"),
        }
    }
    got
}

#[test]
fn a_reply_after_many_polls_is_read_whole_and_in_order() {
    for (fake, proxy) in [(FakeProxy::nym(), Proxy::Nym), (FakeProxy::anon(3), Proxy::Anyone)] {
        let Ok(mut t) = Tunnel::open(far_end(fake), proxy, "example.com", 443) else {
            panic!("did not open")
        };
        assert_eq!(t.write_all(REQUEST), Ok(()));
        assert_eq!(read_all(&mut t), whole());
        assert!(t.ended());
        assert_eq!(t.carrier.to_exit, REQUEST, "the request reached the exit once");
    }
}

#[test]
fn every_exchange_has_the_next_number() {
    let Ok(mut t) = Tunnel::open(far_end(FakeProxy::nym()), Proxy::Nym, "example.com", 443) else {
        panic!("did not open")
    };
    let _ = t.write_all(REQUEST);
    let _ = read_all(&mut t);
    let seqs: Vec<u32> = t.carrier.numbered().iter().map(|(s, _)| *s).collect();
    let want: Vec<u32> = (1..=seqs.len() as u32).collect();
    assert_eq!(seqs, want, "no lost answer, so no number repeats");
}

#[test]
fn a_close_ends_reading_without_asking_again() {
    let Ok(mut t) = Tunnel::open(far_end(FakeProxy::nym()), Proxy::Nym, "example.com", 443) else {
        panic!("did not open")
    };
    let _ = t.write_all(REQUEST);
    let _ = read_all(&mut t);
    let calls = t.carrier.calls;
    let mut buf = [0u8; 16];
    assert_eq!(t.read(&mut buf), Ok(0));
    assert_eq!(t.read_wait(&mut buf, 60_000), Ok(0));
    assert_eq!(t.carrier.calls, calls, "a finished stream is not polled");
    assert_eq!(t.write_all(b"more"), Err(FINISHED));
}

#[test]
fn bytes_that_come_with_the_close_are_still_read() {
    let mut fake = FakeProxy::nym().deliver(2, b"last words");
    fake.close_after = true;
    let Ok(mut t) = Tunnel::open(fake, Proxy::Nym, "example.com", 443) else {
        panic!("did not open")
    };
    let _ = t.write_all(b"x");
    assert!(!t.ended());
    assert_eq!(read_all(&mut t), b"last words");
    assert!(t.ended());
}

#[test]
fn a_lost_answer_is_asked_for_again_and_nothing_is_lost_or_doubled() {
    let mut fake = far_end(FakeProxy::nym());
    fake.lose_next_delivery = true;
    let Ok(mut t) = Tunnel::open(fake, Proxy::Nym, "example.com", 443) else {
        panic!("did not open")
    };
    let _ = t.write_all(REQUEST);
    assert_eq!(read_all(&mut t), whole());
    /* The frame whose answer was lost went again, number and all. */
    let n = t.carrier.numbered();
    let repeats = n.windows(2).filter(|w| w[0] == w[1]).count();
    assert_eq!(repeats, 1);
}

#[test]
fn a_lost_answer_to_a_write_does_not_send_the_bytes_twice() {
    let mut fake = far_end(FakeProxy::anon(2));
    fake.lose_next_write = true;
    let Ok(mut t) = Tunnel::open(fake, Proxy::Anyone, "example.com", 443) else {
        panic!("did not open")
    };
    assert_eq!(t.write_all(REQUEST), Ok(()));
    assert_eq!(t.carrier.to_exit, REQUEST);
    let n = t.carrier.numbered();
    let sent: Vec<_> = n.iter().filter(|(_, b)| b.as_slice() == REQUEST).collect();
    assert_eq!(sent.len(), 2, "the same frame went twice");
    assert_eq!(sent[0], sent[1]);
    assert_eq!(read_all(&mut t), whole());
}

#[test]
fn a_long_write_goes_in_frames_one_record_long() {
    let Ok(mut t) = Tunnel::open(FakeProxy::nym(), Proxy::Nym, "example.com", 443) else {
        panic!("did not open")
    };
    let big: Vec<u8> = (0..40_000u32).map(|i| i as u8).collect();
    assert_eq!(t.write_all(&big), Ok(()));
    assert_eq!(t.carrier.to_exit, big);
    let n = t.carrier.numbered();
    let sizes: Vec<usize> = n.iter().skip(2).map(|(_, b)| b.len()).collect();
    assert_eq!(sizes, [16 * 1024, 16 * 1024, 40_000 - 32 * 1024]);
}

#[test]
fn waiting_for_bytes_that_never_come_is_bounded() {
    let Ok(mut t) = Tunnel::open(FakeProxy::nym(), Proxy::Nym, "example.com", 443) else {
        panic!("did not open")
    };
    let _ = t.write_all(b"x");
    let start = t.carrier.now;
    let mut buf = [0u8; 8];
    assert_eq!(t.read_wait(&mut buf, 5_000), Ok(0));
    let spent = t.carrier.now - start;
    assert!((5_000..5_200).contains(&spent), "{spent}");
    assert!(!t.ended());
}
