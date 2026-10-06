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

//! The peer's window is taken from every acceptable ACK, not only from one
//! that acknowledges new data.

use super::handshake::local_port;
use crate::peer::{connect, fresh, inject, numbers, send, sent, Seg, ACK};

/*
 * A receiver that fell behind closes its window, then opens it again with an
 * ACK of nothing new once its reader catches up (RFC 9293 3.10.7.4: SND.UNA
 * =< SEG.ACK =< SND.NXT updates the window). That ACK was ignored, so the
 * window stayed at zero and the queued bytes were never sent.
 */
#[test]
fn a_window_reopened_by_a_pure_update_resumes_sending() {
    let _g = fresh();
    let h = connect();
    let port = local_port(h);
    let (_, rcv_nxt, _) = numbers(h);
    assert_eq!(send(h, b"a"), 0);
    let (snd_nxt, _, _) = numbers(h);
    let mut closed = Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, &[]);
    closed.window = 0;
    inject(closed);
    crate::peer::drain();
    let _ = sent();
    assert_eq!(send(h, b"bcd"), 0);
    assert!(sent().iter().all(|s| s.payload.is_empty()), "nothing goes into a closed window");
    let mut reopened = Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, &[]);
    reopened.window = 8192;
    inject(reopened);
    crate::peer::drain();
    let out = sent();
    assert!(out.iter().any(|s| s.payload == b"bcd" && s.seq == snd_nxt), "{out:?}");
}

#[test]
fn an_old_ack_does_not_shrink_the_window() {
    let _g = fresh();
    let h = connect();
    let port = local_port(h);
    let (snd_before, rcv_nxt, _) = numbers(h);
    assert_eq!(send(h, b"one"), 0);
    let (snd_nxt, _, _) = numbers(h);
    inject(Seg::from_peer(port, rcv_nxt, snd_nxt, ACK, &[]));
    let mut stale = Seg::from_peer(port, rcv_nxt, snd_before, ACK, &[]);
    stale.window = 0;
    inject(stale);
    crate::peer::drain();
    let _ = sent();
    assert_eq!(send(h, b"two"), 0);
    assert!(sent().iter().any(|s| s.payload == b"two"), "a reordered old ACK closed nothing");
}

/// Three ACKs of nothing new for an unacknowledged "x", with these windows.
fn three_acks_of_nothing_new(windows: [u16; 3]) -> Vec<crate::peer::Seg> {
    let h = connect();
    let port = local_port(h);
    let (_, rcv_nxt, una) = numbers(h);
    assert_eq!(send(h, b"x"), 0);
    let _ = sent();
    for window in windows {
        let mut dup = Seg::from_peer(port, rcv_nxt, una, ACK, &[]);
        dup.window = window;
        inject(dup);
    }
    crate::peer::drain();
    sent()
}

#[test]
fn three_true_duplicate_acks_still_retransmit() {
    let _g = fresh();
    let out = three_acks_of_nothing_new([65535; 3]);
    assert!(out.iter().any(|s| s.payload == b"x"), "fast retransmit: {out:?}");
}

#[test]
fn window_updates_are_not_counted_as_duplicate_acks() {
    let _g = fresh();
    let out = three_acks_of_nothing_new([30000, 40000, 50000]);
    assert!(out.iter().all(|s| s.payload != b"x"), "no fast retransmit: {out:?}");
}
