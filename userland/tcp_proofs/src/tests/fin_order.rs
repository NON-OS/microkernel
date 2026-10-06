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

//! A FIN takes the sequence number after the segment's last data byte, and is
//! only taken once every byte before it has been.

use super::handshake::local_port;
use crate::peer::{connect, fresh, inject, numbers, recv, sent, state_of, Seg, ACK, FIN, PSH};

const ESTABLISHED: u8 = 3;
const CLOSE_WAIT: u8 = 4;

#[test]
fn a_fin_with_data_is_acknowledged_after_the_data() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let _ = sent();
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt, ACK | PSH | FIN, b"bye"));
    let (errno, data) = recv(h);
    assert_eq!((errno, data.as_slice()), (0, b"bye".as_slice()));
    assert_eq!(state_of(h), Some(CLOSE_WAIT));
    assert_eq!(numbers(h).1, rcv_nxt.wrapping_add(4));
}

/*
 * A retransmission the peer packed again: two bytes already taken, three new
 * ones and the FIN. The segment was dropped for starting before RCV.NXT, yet
 * its FIN was taken at RCV.NXT, so the ACK claimed the first of the three new
 * bytes and the connection went to CLOSE-WAIT having lost all three.
 */
#[test]
fn a_fin_on_an_overlapping_retransmission_follows_its_data() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let port = local_port(h);
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, b"ab"));
    assert_eq!(recv(h), (0, b"ab".to_vec()));
    let _ = sent();
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK | FIN, b"abcde"));
    let (errno, data) = recv(h);
    assert_eq!((errno, data.as_slice()), (0, b"cde".as_slice()), "the new bytes are kept");
    assert_eq!(numbers(h).1, rcv_nxt.wrapping_add(6), "five data bytes, then the FIN");
    assert_eq!(state_of(h), Some(CLOSE_WAIT));
}

#[test]
fn a_fin_ahead_of_a_gap_is_not_taken() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    inject(Seg::from_peer(local_port(h), rcv_nxt.wrapping_add(10), snd_nxt, ACK | FIN, b"late"));
    crate::peer::drain();
    assert_eq!(state_of(h), Some(ESTABLISHED));
    assert_eq!(numbers(h).1, rcv_nxt);
}

const TIME_WAIT: u8 = 8;

/*
 * Closing our side leaves the peer's side open (RFC 9293 3.6): data and a FIN
 * that arrive in FIN-WAIT are the rest of its stream. The data was dropped
 * and the FIN taken at SEG.SEQ + 1, so the ACK covered bytes never delivered.
 */
#[test]
fn data_and_a_fin_after_our_close_are_delivered_in_order() {
    let _g = fresh();
    let h = connect();
    let (_, rcv_nxt, _) = numbers(h);
    assert_eq!(crate::peer::request(crate::protocol::OP_CLOSE, &h.to_le_bytes()).0, 0);
    let (snd_nxt, _, _) = numbers(h);
    let _ = sent();
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt, ACK | FIN, b"tail"));
    crate::peer::drain();
    assert_eq!(state_of(h), Some(TIME_WAIT));
    assert_eq!(numbers(h).1, rcv_nxt.wrapping_add(5), "four data bytes, then the FIN");
    let acks = sent();
    assert!(acks.iter().any(|s| s.ack == rcv_nxt.wrapping_add(5)), "{acks:?}");
    assert_eq!(recv(h), (0, b"tail".to_vec()));
}

#[test]
fn data_before_the_peer_fin_in_fin_wait_2_is_delivered() {
    let _g = fresh();
    let h = connect();
    let port = local_port(h);
    let (_, rcv_nxt, _) = numbers(h);
    assert_eq!(crate::peer::request(crate::protocol::OP_CLOSE, &h.to_le_bytes()).0, 0);
    let (snd_nxt, _, _) = numbers(h);
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, &[]));
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, b"more"));
    inject(Seg::from_peer(port, rcv_nxt.wrapping_add(4), snd_nxt, ACK | FIN, &[]));
    crate::peer::drain();
    assert_eq!(state_of(h), Some(TIME_WAIT));
    assert_eq!(numbers(h).1, rcv_nxt.wrapping_add(5));
    assert_eq!(recv(h), (0, b"more".to_vec()));
}
