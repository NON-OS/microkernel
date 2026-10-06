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

//! A segment whose ACK field is not one this side could have earned is
//! dropped, data and all, and answered with an ACK (RFC 9293 3.10.7.4,
//! RFC 5961 section 5.2).

use super::handshake::local_port;
use crate::peer::{connect, fresh, inject, numbers, recv, sent, Seg, ACK};

/*
 * Data from a sender that claims to have seen bytes this side never sent is
 * not from the peer: a blind injector has the sequence number in a window's
 * reach but no idea where SND.NXT is. The data was delivered all the same.
 */
#[test]
fn data_acknowledging_bytes_never_sent_is_dropped() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let _ = sent();
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt.wrapping_add(1000), ACK, b"inject"));
    assert_ne!(recv(h).0, 0, "nothing was delivered");
    assert_eq!(numbers(h).1, rcv_nxt);
    let out = sent();
    assert!(out.iter().any(|s| s.flags == ACK && s.ack == rcv_nxt), "answered with an ACK");
}

#[test]
fn data_acknowledging_far_in_the_past_is_dropped() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let ack = snd_nxt.wrapping_sub(0x0100_0000);
    inject(Seg::from_peer(local_port(h), rcv_nxt, ack, ACK, b"inject"));
    assert_ne!(recv(h).0, 0);
    assert_eq!(numbers(h).1, rcv_nxt);
}

#[test]
fn data_with_an_ack_inside_the_range_is_taken() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    inject(Seg::from_peer(local_port(h), rcv_nxt, snd_nxt.wrapping_sub(5), ACK, b"fine"));
    assert_eq!(recv(h), (0, b"fine".to_vec()));
}

#[test]
fn data_without_an_ack_is_dropped() {
    let _g = fresh();
    let h = connect();
    let (_, rcv_nxt, _) = numbers(h);
    inject(Seg::from_peer(local_port(h), rcv_nxt, 0, crate::peer::PSH, b"no ack"));
    assert_ne!(recv(h).0, 0, "nothing was delivered");
    assert_eq!(numbers(h).1, rcv_nxt);
}
