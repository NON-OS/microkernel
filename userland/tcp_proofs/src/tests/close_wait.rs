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

//! CLOSE-WAIT: the peer has finished sending, but this side has not, and
//! what the peer still says about this side's data counts.

use super::handshake::local_port;
use crate::peer::{connect, drain, fresh, inject, numbers, send, sent, state_of, Seg, ACK, FIN};

const CLOSE_WAIT: u8 = 4;

/// A connection in CLOSE-WAIT with "x" sent and not yet acknowledged.
fn half_closed() -> (u32, u32, u32) {
    let h = connect();
    let (before_x, rcv_nxt, _) = numbers(h);
    assert_eq!(send(h, b"x"), 0);
    inject(Seg::from_peer(local_port(h), rcv_nxt, before_x, ACK | FIN, &[]));
    drain();
    assert_eq!(state_of(h), Some(CLOSE_WAIT));
    let _ = sent();
    (h, before_x.wrapping_add(1), rcv_nxt.wrapping_add(1))
}

/*
 * The peer's ACK of data this side sent was ignored once the peer had sent
 * its FIN, so the data stayed queued for retransmission: it went out again on
 * every timeout until the retry limit tore the connection down, though it
 * had arrived.
 */
#[test]
fn an_ack_in_close_wait_releases_what_it_acknowledges() {
    let _g = fresh();
    let (h, snd_nxt, rcv_nxt) = half_closed();
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt, ACK, &[]));
    drain();
    assert_eq!(numbers(h).2, snd_nxt, "SND.UNA moved past \"x\"");
    nonos_libc::advance(5_000);
    crate::server::retransmit::scan(crate::clock::now_ms());
    assert!(sent().iter().all(|s| s.payload != b"x"), "nothing acknowledged is sent again");
}

/*
 * When this side's ACK of the peer's FIN is lost, the peer sends the FIN
 * again and waits for that ACK. In CLOSE-WAIT every segment was ignored, so
 * it never came and the peer sat in FIN-WAIT-1 until it gave up.
 */
#[test]
fn a_repeated_fin_in_close_wait_is_acknowledged_again() {
    let _g = fresh();
    let (h, snd_nxt, rcv_nxt) = half_closed();
    inject(Seg::from_peer(local_port(h), rcv_nxt.wrapping_sub(1), snd_nxt, ACK | FIN, &[]));
    drain();
    let out = sent();
    assert!(out.iter().any(|s| s.flags == ACK && s.ack == rcv_nxt), "{out:?}");
    assert_eq!(state_of(h), Some(CLOSE_WAIT));
}
