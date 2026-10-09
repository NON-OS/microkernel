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

//! A segment that starts before RCV.NXT and runs past it gives up its new
//! bytes, not nothing (RFC 9293 3.10.7.4, trimming to the window).

use super::handshake::local_port;
use crate::peer::{connect, fresh, inject, numbers, recv, Seg, ACK};

#[test]
fn the_new_tail_of_an_overlapping_segment_is_kept() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let port = local_port(h);
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, b"head"));
    assert_eq!(recv(h), (0, b"head".to_vec()));
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, b"head and tail"));
    assert_eq!(recv(h), (0, b" and tail".to_vec()), "only the bytes not yet taken");
    assert_eq!(numbers(h).1, rcv_nxt.wrapping_add(13));
}

#[test]
fn a_whole_duplicate_gives_nothing_twice() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let port = local_port(h);
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, b"once"));
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, b"once"));
    assert_eq!(recv(h), (0, b"once".to_vec()));
    assert_ne!(recv(h).0, 0, "the duplicate adds nothing");
    assert_eq!(numbers(h).1, rcv_nxt.wrapping_add(4));
}
