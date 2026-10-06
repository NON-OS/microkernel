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

//! An active open, data both ways and a close, against the played peer: the
//! harness itself, proven on the path every connection takes.

use crate::peer::{connect, fresh, inject, numbers, recv, send, sent, state_of, Seg};
use crate::peer::{ACK, PEER_ISS, PSH, SYN};

const ESTABLISHED: u8 = 3;

#[test]
fn a_connect_completes_the_three_way_handshake() {
    let _g = fresh();
    let h = connect();
    assert_eq!(state_of(h), Some(ESTABLISHED));
    let out = sent();
    assert_eq!(out.len(), 2, "SYN, then the ACK of the SYN-ACK: {out:?}");
    assert_eq!(out[0].flags, SYN);
    assert_eq!(out[1].flags, ACK);
    assert_eq!(out[1].seq, out[0].seq.wrapping_add(1));
    assert_eq!(out[1].ack, PEER_ISS.wrapping_add(1));
}

#[test]
fn data_in_order_reaches_the_reader_and_is_acknowledged() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let _ = sent();
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt, ACK | PSH, b"hello"));
    let (errno, data) = recv(h);
    assert_eq!((errno, data.as_slice()), (0, b"hello".as_slice()));
    let acks = sent();
    assert!(acks.iter().any(|s| s.has(ACK) && s.ack == rcv_nxt.wrapping_add(5)), "{acks:?}");
}

#[test]
fn data_written_goes_out_as_a_segment() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, _, _) = numbers(h);
    let _ = sent();
    assert_eq!(send(h, b"GET / HTTP/1.1\r\n\r\n"), 0);
    let out = sent();
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!(out[0].seq, snd_nxt);
    assert_eq!(out[0].payload, b"GET / HTTP/1.1\r\n\r\n");
}

/// The local port a connection was given.
pub(crate) fn local_port(handle: u32) -> u16 {
    let mut t = crate::state::TABLE.lock();
    t.by_handle_mut(handle).expect("connection").tcb.local.port
}
