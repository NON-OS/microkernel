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

//! Half-open connections, a SYN answered and the handshake never finished,
//! hold a listener's few places for a bounded time and never the machine's.

use crate::peer::{connect, fresh, request, sent, set_auto, state_of, Auto, Seg};
use crate::peer::{inject, ACK, PEER_ISS, SYN};
use crate::protocol::{OP_ACCEPT, OP_LISTEN};
use crate::server::tcp_rx::drain_one;
use crate::server::tick::tick;
use crate::state::TABLE;
use crate::tcp::{State, HALF_OPEN_MAX, HALF_OPEN_MS, MAX_CONN_PER_PID};

const PORT: u16 = 2222;
const ESTABLISHED: u8 = 3;
const START_MS: u64 = 1_000_000;

fn listen() -> u32 {
    let (errno, body) = request(OP_LISTEN, &PORT.to_le_bytes());
    assert_eq!(errno, 0, "listen");
    u32::from_le_bytes([body[0], body[1], body[2], body[3]])
}

/// A SYN from `sport` on the remote host, as a forger or a slow client sends.
fn syn_from(sport: u16) {
    let mut s = Seg::from_peer(PORT, PEER_ISS, 0, SYN, &[]);
    s.sport = sport;
    inject(s);
    while drain_one() {}
}

/// The ACK that finishes `sport`'s handshake, from the SYN-ACK it was sent.
fn finish(sport: u16, synack_seq: u32) {
    let mut s =
        Seg::from_peer(PORT, PEER_ISS.wrapping_add(1), synack_seq.wrapping_add(1), ACK, &[]);
    s.sport = sport;
    inject(s);
    while drain_one() {}
}

fn synack_to(sport: u16) -> u32 {
    let out = sent();
    out.iter()
        .find(|s| s.dport == sport && s.has(SYN) && s.has(ACK))
        .map(|s| s.seq)
        .expect("a SYN-ACK went to the peer")
}

/// The remote ports of the half-open connections the capsule holds.
fn half_open() -> Vec<u16> {
    let mut t = TABLE.lock();
    t.entries_mut()
        .iter()
        .filter(|e| e.tcb.state == State::SynReceived)
        .map(|e| e.tcb.remote.port)
        .collect()
}

fn at(ms: u64) {
    nonos_libc::set_time(ms as i64);
}

#[test]
#[allow(clippy::assertions_on_constants)] // the bound is the point
fn the_half_open_bound_leaves_room_for_every_other_connection() {
    assert!(HALF_OPEN_MAX + 1 < MAX_CONN_PER_PID, "a listener and its half-open entries fit");
}

/*
 * Every application's sockets reach net.tcp as one owner. Before, each SYN
 * left an entry that never ended, so at most thirty-one forged SYNs to any
 * listening port refused every later connection, outgoing ones included.
 */
#[test]
fn a_syn_flood_holds_a_listener_s_places_and_never_the_machine_s() {
    let _g = fresh();
    set_auto(Auto { answer_synack: false, ..Auto::plain() });
    listen();
    for sport in 1000..1000 + 4 * MAX_CONN_PER_PID as u16 {
        syn_from(sport);
    }
    assert_eq!(half_open().len(), HALF_OPEN_MAX, "the flood holds only its places");
    let _ = sent();
    set_auto(Auto::plain());
    let c = connect();
    assert_eq!(state_of(c), Some(ESTABLISHED), "an outgoing connection still opens");
}

#[test]
fn a_new_syn_past_the_bound_takes_the_oldest_place() {
    let _g = fresh();
    set_auto(Auto { answer_synack: false, ..Auto::plain() });
    let l = listen();
    let first = 3000u16;
    for sport in first..first + HALF_OPEN_MAX as u16 {
        syn_from(sport);
    }
    let _ = sent();
    let late = first + HALF_OPEN_MAX as u16;
    syn_from(late);
    let held = half_open();
    assert_eq!(held.len(), HALF_OPEN_MAX);
    assert!(!held.contains(&first), "the oldest gave way: {held:?}");
    assert!(held.contains(&late), "the newest was taken: {held:?}");
    finish(late, synack_to(late));
    let (errno, body) = request(OP_ACCEPT, &l.to_le_bytes());
    assert_eq!(errno, 0, "the client that answered is accepted");
    let c = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
    assert_eq!(state_of(c), Some(ESTABLISHED));
}

#[test]
fn a_half_open_connection_ends_when_its_window_does() {
    let _g = fresh();
    set_auto(Auto { answer_synack: false, ..Auto::plain() });
    listen();
    at(START_MS);
    syn_from(4000);
    at(START_MS + HALF_OPEN_MS - 1);
    tick();
    assert_eq!(half_open(), vec![4000], "still inside its window");
    at(START_MS + HALF_OPEN_MS);
    tick();
    assert!(half_open().is_empty(), "the unfinished handshake ended");
}

#[test]
fn a_finished_handshake_outlives_the_window() {
    let _g = fresh();
    set_auto(Auto { answer_synack: false, ..Auto::plain() });
    let l = listen();
    at(START_MS);
    syn_from(5000);
    finish(5000, synack_to(5000));
    let (errno, body) = request(OP_ACCEPT, &l.to_le_bytes());
    assert_eq!(errno, 0);
    let c = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
    at(START_MS + 4 * HALF_OPEN_MS);
    tick();
    assert_eq!(state_of(c), Some(ESTABLISHED), "the window applies to half-open entries only");
}

#[test]
fn a_repeated_syn_keeps_its_one_place() {
    let _g = fresh();
    set_auto(Auto { answer_synack: false, ..Auto::plain() });
    listen();
    for _ in 0..3 * HALF_OPEN_MAX {
        syn_from(6000);
    }
    assert_eq!(half_open(), vec![6000], "a retransmitted SYN answers again from the same entry");
}
