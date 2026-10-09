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

//! What the stack keeps after an application closes a connection: a
//! FIN-WAIT-2 that ends when its peer never finishes, and TIME-WAIT entries
//! that never hold the owner's places.

use super::handshake::local_port;
use crate::peer::{connect, fresh, inject, numbers, request, sent, state_of, Seg, ACK, FIN};
use crate::protocol::OP_CLOSE;
use crate::server::tick::tick;
use crate::state::TABLE;
use crate::tcp::{msl_2_ms, State, FIN_WAIT_2_MS, MAX_CONN_PER_PID};

const FIN_WAIT_2: u8 = 6;
const TIME_WAIT: u8 = 8;
const T0: u64 = 1_000_000;
/// TABLE_CAP in capsule_net_tcp/src/state/table/types.rs.
const TABLE_CAP: usize = 256;

fn at(ms: u64) {
    nonos_libc::set_time(ms as i64);
}

/// Close `h` and have the peer acknowledge our FIN, adding its own when `fin`.
fn close_acked(h: u32, fin: bool) {
    let (_, rcv_nxt, _) = numbers(h);
    assert_eq!(request(OP_CLOSE, &h.to_le_bytes()).0, 0, "close");
    let (snd_nxt, _, _) = numbers(h);
    let flags = if fin { ACK | FIN } else { ACK };
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt, flags, &[]));
    crate::peer::drain();
    let _ = sent();
}

fn in_time_wait() -> usize {
    TABLE.lock().entries_mut().iter().filter(|e| e.tcb.state == State::TimeWait).count()
}

/*
 * The peer acknowledged our FIN and never sent its own: it crashed, or a NAT
 * forgot the flow. The entry waited for that FIN for ever, holding one of
 * the owner's places.
 */
#[test]
fn fin_wait_2_ends_when_the_peer_never_finishes() {
    let _g = fresh();
    at(T0);
    let h = connect();
    close_acked(h, false);
    assert_eq!(state_of(h), Some(FIN_WAIT_2));
    at(T0 + FIN_WAIT_2_MS - 1);
    tick();
    assert_eq!(state_of(h), Some(FIN_WAIT_2), "still inside its wait");
    at(T0 + FIN_WAIT_2_MS);
    tick();
    assert_eq!(state_of(h), None, "the connection the peer never finished is gone");
}

#[test]
fn a_fin_in_fin_wait_2_gets_its_whole_time_wait() {
    let _g = fresh();
    at(T0);
    let h = connect();
    close_acked(h, false);
    let fin_at = T0 + FIN_WAIT_2_MS - 1_000;
    at(fin_at);
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt, ACK | FIN, &[]));
    crate::peer::drain();
    assert_eq!(state_of(h), Some(TIME_WAIT));
    at(T0 + FIN_WAIT_2_MS);
    tick();
    assert_eq!(state_of(h), Some(TIME_WAIT), "the FIN-WAIT-2 deadline no longer applies");
    at(fin_at + msl_2_ms() - 1);
    tick();
    assert_eq!(state_of(h), Some(TIME_WAIT));
    at(fin_at + msl_2_ms());
    tick();
    assert_eq!(state_of(h), None, "twice the MSL after the FIN");
}

/*
 * Every application's sockets reach net.tcp as one owner, and each closed
 * connection sat in TIME-WAIT for a minute counted against it, so the
 * thirty-third connection closed within a minute refused every new one.
 */
#[test]
fn time_wait_holds_none_of_the_owner_s_places() {
    let _g = fresh();
    at(T0);
    for _ in 0..2 * MAX_CONN_PER_PID {
        let h = connect();
        close_acked(h, true);
        assert_eq!(state_of(h), Some(TIME_WAIT));
    }
    assert_eq!(in_time_wait(), 2 * MAX_CONN_PER_PID);
    let h = connect();
    assert_eq!(state_of(h), Some(3), "a new connection still opens");
}

#[test]
fn a_full_table_takes_the_oldest_time_wait_s_place() {
    let _g = fresh();
    at(T0);
    let mut handles = Vec::new();
    for i in 0..TABLE_CAP + 40 {
        at(T0 + i as u64);
        let h = connect();
        close_acked(h, true);
        handles.push(h);
        assert!(TABLE.lock().entries_mut().len() <= TABLE_CAP, "the table stays bounded");
    }
    assert_eq!(state_of(handles[0]), None, "the oldest gave its place");
    assert_eq!(state_of(handles[39]), None);
    assert_eq!(state_of(handles[40]), Some(TIME_WAIT), "the rest keep their wait");
    assert_eq!(state_of(*handles.last().unwrap()), Some(TIME_WAIT));
}
