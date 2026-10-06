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

//! A closed window is probed until it opens, and a reader's room is
//! announced. On the old capsule a lost reopening left queued data unsent
//! for good, and a reader that drained a full queue told the peer nothing.

use super::handshake::local_port;
use crate::peer::{connect, drain, fresh, inject, numbers, recv, send, sent, state_of, Seg, ACK};
use crate::tcp::persist::PERSIST_MAX_MS;
use crate::state::RX_DEPTH;
use crate::tcp::{MAX_RETX, MSS};

fn now() -> u64 {
    crate::clock::now_ms()
}

fn at(ms: u64) {
    nonos_libc::set_time(ms as i64);
    crate::server::tick::tick();
}

fn rto(h: u32) -> u64 {
    let mut t = crate::state::TABLE.lock();
    u64::from(t.by_handle_mut(h).expect("open").rtt.rto_ms())
}

/// A connection whose peer took "a" and closed its window, with "bcd"
/// queued behind it; returns the handle and the peer's view of it.
fn held_back() -> (u32, u16, u32, u32) {
    let h = connect();
    let port = local_port(h);
    let (_, rcv_nxt, _) = numbers(h);
    assert_eq!(send(h, b"a"), 0);
    let (snd_nxt, _, _) = numbers(h);
    let mut closed = Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, &[]);
    closed.window = 0;
    inject(closed);
    drain();
    assert_eq!(send(h, b"bcd"), 0);
    let _ = sent();
    (h, port, rcv_nxt, snd_nxt)
}

fn probes(out: &[Seg], una: u32) -> usize {
    out.iter().filter(|s| s.payload.is_empty() && s.has(ACK) && s.seq == una.wrapping_sub(1)).count()
}

#[test]
fn a_closed_window_is_probed_and_the_data_goes_when_it_opens() {
    let _g = fresh();
    let (h, port, rcv_nxt, una) = held_back();
    let start = now();
    at(start);
    let wait = rto(h);
    at(start + wait - 1);
    assert_eq!(probes(&sent(), una), 0, "no probe before the first wait");
    at(start + wait);
    let out = sent();
    assert_eq!(probes(&out, una), 1, "one probe at SND.UNA - 1: {out:?}");
    assert!(out.iter().all(|s| s.payload.is_empty()), "a probe takes nothing from the queue");
    let mut opened = Seg::from_peer(port, rcv_nxt, una, ACK, &[]);
    opened.window = 8192;
    inject(opened);
    drain();
    assert!(sent().iter().any(|s| s.payload == b"bcd" && s.seq == una), "the queue goes once open");
}

#[test]
fn probes_back_off_and_a_peer_that_answers_is_kept() {
    let _g = fresh();
    let (h, port, rcv_nxt, una) = held_back();
    let mut t = now();
    at(t);
    let mut wait = rto(h);
    let mut waits = Vec::new();
    for _ in 0..(MAX_RETX as usize + 6) {
        at(t + wait - 1);
        assert_eq!(probes(&sent(), una), 0, "early at {wait}");
        t += wait;
        at(t);
        assert_eq!(probes(&sent(), una), 1, "the probe after {wait} ms");
        waits.push(wait);
        let mut still_closed = Seg::from_peer(port, rcv_nxt, una, ACK, &[]);
        still_closed.window = 0;
        inject(still_closed);
        drain();
        wait = (wait * 2).min(PERSIST_MAX_MS);
    }
    assert!(waits.windows(2).all(|w| w[1] == (w[0] * 2).min(PERSIST_MAX_MS)), "{waits:?}");
    assert_eq!(*waits.last().unwrap(), PERSIST_MAX_MS, "the wait stops at the cap");
    assert!(state_of(h).is_some(), "a peer answering every probe keeps its connection");
}

#[test]
fn a_peer_that_never_answers_is_dropped() {
    let _g = fresh();
    let (h, _, _, una) = held_back();
    let mut t = now();
    at(t);
    let mut wait = rto(h);
    for _ in 0..MAX_RETX {
        t += wait;
        at(t);
        assert_eq!(probes(&sent(), una), 1);
        wait = (wait * 2).min(PERSIST_MAX_MS);
    }
    assert!(state_of(h).is_some(), "still there after {MAX_RETX} unanswered probes");
    at(t + wait);
    assert!(state_of(h).is_none(), "dropped after the last probe went unanswered");
}

/// The window in the last segment the capsule sent that carries no data.
fn last_window(out: &[Seg]) -> Option<u16> {
    out.iter().rev().find(|s| s.payload.is_empty() && s.has(ACK)).map(|s| s.window)
}

#[test]
fn a_reader_that_drains_a_full_queue_announces_the_room() {
    let _g = fresh();
    let h = connect();
    let port = local_port(h);
    let (snd_nxt, mut rcv_nxt, _) = numbers(h);
    for i in 0..RX_DEPTH {
        let data = vec![i as u8; MSS];
        inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, &data));
        rcv_nxt = rcv_nxt.wrapping_add(MSS as u32);
    }
    drain();
    assert_eq!(last_window(&sent()), Some(0), "a full queue advertises no room");
    let (errno, got) = recv(h);
    assert_eq!((errno, got.len()), (0, MSS));
    let out = sent();
    assert_eq!(last_window(&out), Some(MSS as u16), "the read's room is announced: {out:?}");
    assert!(out.iter().all(|s| s.payload.is_empty()), "an announcement carries no data");
}

#[test]
fn a_reader_keeping_up_sends_no_extra_ack() {
    let _g = fresh();
    let h = connect();
    let port = local_port(h);
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, &[7u8; 100]));
    drain();
    let _ = sent();
    let (errno, got) = recv(h);
    assert_eq!((errno, got.len()), (0, 100));
    assert!(sent().is_empty(), "the window was nearly open; the read needs no update");
}
