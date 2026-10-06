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

//! The connections of an owner that ended without closing them are reset and
//! freed; a living owner's, and TIME-WAIT, are left alone.

use super::handshake::local_port;
use crate::peer::{connect, fresh, request, sent, set_auto, state_of, Auto, Seg, APP};
use crate::peer::{inject, PEER_ISS, RST, SYN};
use crate::protocol::OP_LISTEN;
use crate::server::orphans::reap_now;
use crate::state::TABLE;
use crate::tcp::State;

const TIME_WAIT: u8 = 8;

fn rsts_to(port: u16) -> usize {
    sent().iter().filter(|s| s.has(RST) && s.dport == port).count()
}

/*
 * Only the owner can close or read its connections, so a terminal that
 * crashed left every one open for good, each holding a place and leaving its
 * peer waiting on a connection nobody answers.
 */
#[test]
fn an_ended_owner_s_connections_are_reset_and_freed() {
    let _g = fresh();
    let a = connect();
    let b = connect();
    let _ = sent();
    nonos_libc::end_pid(APP);
    reap_now();
    assert_eq!(state_of(a), None);
    assert_eq!(state_of(b), None);
    let out = sent();
    assert_eq!(out.iter().filter(|s| s.has(RST)).count(), 2, "each peer is told: {out:?}");
    assert!(TABLE.lock().entries_mut().is_empty());
}

#[test]
fn a_living_owner_keeps_its_connections() {
    let _g = fresh();
    let a = connect();
    let _ = sent();
    nonos_libc::end_pid(APP + 1);
    reap_now();
    assert_eq!(state_of(a), Some(3));
    assert_eq!(rsts_to(local_port(a)), 0);
}

#[test]
fn an_ended_owner_s_listener_and_half_open_children_go_too() {
    let _g = fresh();
    set_auto(Auto { answer_synack: false, ..Auto::plain() });
    let (errno, _) = request(OP_LISTEN, &7070u16.to_le_bytes());
    assert_eq!(errno, 0);
    inject(Seg::from_peer(7070, PEER_ISS, 0, SYN, &[]));
    crate::peer::drain();
    let _ = sent();
    nonos_libc::end_pid(APP);
    reap_now();
    assert!(TABLE.lock().entries_mut().is_empty(), "no listener and no half-open child left");
    assert_eq!(sent().iter().filter(|s| s.has(RST)).count(), 1, "the half-open peer is told");
}

#[test]
fn time_wait_ends_by_itself_even_for_an_ended_owner() {
    let _g = fresh();
    let h = connect();
    let (_, rcv_nxt, _) = crate::peer::numbers(h);
    assert_eq!(request(crate::protocol::OP_CLOSE, &h.to_le_bytes()).0, 0);
    let (snd_nxt, _, _) = crate::peer::numbers(h);
    let ackfin = crate::peer::ACK | crate::peer::FIN;
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt, ackfin, &[]));
    crate::peer::drain();
    assert_eq!(state_of(h), Some(TIME_WAIT));
    let _ = sent();
    nonos_libc::end_pid(APP);
    reap_now();
    let left = TABLE.lock().entries_mut().iter().filter(|e| e.tcb.state == State::TimeWait).count();
    assert_eq!(left, 1, "TIME-WAIT still absorbs the old connection's late segments");
    assert_eq!(rsts_to(local_port(h)), 0);
}
