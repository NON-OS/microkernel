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

//! A connection accepted on a listener can send, and takes what the peer
//! sent with the ACK that completed it.

use crate::peer::{fresh, inject, numbers, recv, request, send, sent, state_of, Seg};
use crate::peer::{set_auto, Auto, ACK, FIN, PEER_ISS, PSH, SYN};
use crate::protocol::{OP_ACCEPT, OP_LISTEN};

const PORT: u16 = 8080;
const ESTABLISHED: u8 = 3;
const CLOSE_WAIT: u8 = 4;

fn listen() -> u32 {
    let (errno, body) = request(OP_LISTEN, &PORT.to_le_bytes());
    assert_eq!(errno, 0, "listen");
    u32::from_le_bytes([body[0], body[1], body[2], body[3]])
}

fn accept(listener: u32) -> u32 {
    let (errno, body) = request(OP_ACCEPT, &listener.to_le_bytes());
    assert_eq!(errno, 0, "accept");
    u32::from_le_bytes([body[0], body[1], body[2], body[3]])
}

/// The SYN-ACK the capsule answered with, once the peer's SYN is taken.
fn syn_ack() -> Seg {
    let out = sent();
    out.into_iter().find(|s| s.has(SYN) && s.has(ACK)).expect("a SYN-ACK went out")
}

/*
 * RFC 9293 3.10.7.2: a listener's new connection starts with SND.UNA = ISS,
 * and the ACK of its SYN moves SND.UNA past it. It started at zero and the
 * completing ACK was not taken as an acknowledgement, so the bytes in flight
 * read as the whole ISS and the send window was never open: a server that
 * speaks first (SMTP, SSH, a banner) never sent a byte.
 */
#[test]
fn an_accepted_connection_can_send_first() {
    let _g = fresh();
    let l = listen();
    inject(Seg::from_peer(PORT, PEER_ISS, 0, SYN, &[]));
    let c = accept(l);
    assert_eq!(state_of(c), Some(ESTABLISHED));
    let (snd_nxt, _, snd_una) = numbers(c);
    assert_eq!(snd_una, snd_nxt, "the SYN is acknowledged: SND.UNA = ISS + 1 = SND.NXT");
    let _ = sent();
    assert_eq!(send(c, b"220 ready\r\n"), 0);
    let out = sent();
    assert!(out.iter().any(|s| s.payload == b"220 ready\r\n"), "the banner went out: {out:?}");
}

#[test]
fn data_riding_on_the_completing_ack_is_kept() {
    let _g = fresh();
    set_auto(Auto { answer_synack: false, ..Auto::plain() });
    let l = listen();
    inject(Seg::from_peer(PORT, PEER_ISS, 0, SYN, &[]));
    crate::server::tcp_rx::drain_one();
    let sa = syn_ack();
    let first = PEER_ISS.wrapping_add(1);
    inject(Seg::from_peer(PORT, first, sa.seq.wrapping_add(1), ACK | PSH, b"EHLO"));
    let c = accept(l);
    let (errno, data) = recv(c);
    assert_eq!((errno, data.as_slice()), (0, b"EHLO".as_slice()));
    assert_eq!(numbers(c).1, first.wrapping_add(4), "RCV.NXT is past the data");
}

#[test]
fn a_fin_with_data_on_the_completing_ack_is_counted_after_the_data() {
    let _g = fresh();
    set_auto(Auto { answer_synack: false, ..Auto::plain() });
    let l = listen();
    inject(Seg::from_peer(PORT, PEER_ISS, 0, SYN, &[]));
    crate::server::tcp_rx::drain_one();
    let sa = syn_ack();
    let first = PEER_ISS.wrapping_add(1);
    inject(Seg::from_peer(PORT, first, sa.seq.wrapping_add(1), ACK | PSH | FIN, b"QUIT"));
    let c = accept(l);
    assert_eq!(state_of(c), Some(CLOSE_WAIT));
    assert_eq!(numbers(c).1, first.wrapping_add(5), "four data bytes, then the FIN");
    let (errno, data) = recv(c);
    assert_eq!((errno, data.as_slice()), (0, b"QUIT".as_slice()));
}
