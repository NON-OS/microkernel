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

//! Which resets end a connection (RFC 9293 3.10.7.3, RFC 5961 section 3).

use super::handshake::local_port;
use crate::peer::{connect, fresh, inject, numbers, request, sent, set_auto, state_of, Auto, Seg};
use crate::peer::{ACK, PEER_PORT, REMOTE, RST, SYN};
use crate::protocol::{E_TIMEOUT, OP_CONNECT};

const ESTABLISHED: u8 = 3;

fn connect_errno() -> u16 {
    let mut body = REMOTE.to_vec();
    body.extend_from_slice(&PEER_PORT.to_le_bytes());
    request(OP_CONNECT, &body).0
}

/*
 * In SYN-SENT there is no receive sequence yet, so a reset is believed only
 * when it acknowledges the SYN. A bare reset was checked against RCV.NXT,
 * still zero, so an off-path sender's RST with a small sequence number killed
 * every connection attempt before the real SYN-ACK arrived.
 */
#[test]
fn a_reset_that_does_not_acknowledge_the_syn_is_ignored() {
    let _g = fresh();
    set_auto(Auto { forged_rst: Some(0), ..Auto::plain() });
    assert_eq!(connect_errno(), 0, "the real SYN-ACK behind the forged RST completes it");
}

/*
 * A refusal acknowledges the SYN, and ends the attempt there and then. It
 * was believed only when its sequence number fell in a window that does not
 * exist yet, and even then the caller waited out the whole eight seconds.
 */
#[test]
fn a_refused_connect_fails_at_once() {
    for seq in [0u32, 0x1234_5678] {
        let _g = fresh();
        set_auto(Auto { refuse: Some(seq), ..Auto::plain() });
        let start = nonos_libc::mk_time_millis();
        assert_eq!(connect_errno(), E_TIMEOUT);
        let waited = nonos_libc::mk_time_millis() - start;
        assert!(waited < 100, "refused with seq {seq:#x} after {waited} ms");
    }
}

#[test]
fn a_reset_at_rcv_nxt_ends_the_connection() {
    let _g = fresh();
    let h = connect();
    let (_, rcv_nxt, _) = numbers(h);
    inject(Seg::from_peer(local_port(h), rcv_nxt, 0, RST, &[]));
    crate::peer::drain();
    assert_eq!(state_of(h), None, "the connection is gone");
}

/*
 * RFC 5961 3.2: inside the window but not at RCV.NXT, a reset is answered
 * with a challenge ACK and the connection stays. A blind sender then needs
 * the exact sequence number, not one of a window's worth.
 */
#[test]
fn a_reset_elsewhere_in_the_window_draws_a_challenge_ack() {
    let _g = fresh();
    let h = connect();
    let (snd_nxt, rcv_nxt, _) = numbers(h);
    let _ = sent();
    inject(Seg::from_peer(local_port(h), rcv_nxt.wrapping_add(100), 0, RST, &[]));
    crate::peer::drain();
    assert_eq!(state_of(h), Some(ESTABLISHED));
    let out = sent();
    assert_eq!(out.len(), 1, "one challenge ACK: {out:?}");
    assert_eq!((out[0].flags, out[0].seq, out[0].ack), (ACK, snd_nxt, rcv_nxt));
}

#[test]
fn a_reset_outside_the_window_is_dropped() {
    let _g = fresh();
    let h = connect();
    let (_, rcv_nxt, _) = numbers(h);
    let _ = sent();
    inject(Seg::from_peer(local_port(h), rcv_nxt.wrapping_add(0x4000_0000), 0, RST, &[]));
    inject(Seg::from_peer(local_port(h), rcv_nxt.wrapping_sub(1), 0, RST, &[]));
    crate::peer::drain();
    assert_eq!(state_of(h), Some(ESTABLISHED));
    assert!(sent().is_empty(), "nothing answers a reset from outside the window");
}

/*
 * A SYN with data to a closed port (TCP Fast Open) is refused by a reset that
 * acknowledges the SYN and the data. One that acknowledged SEG.SEQ + 1 only
 * is not acceptable to the sender, which then waits out its own timeout
 * instead of hearing "refused".
 */
#[test]
fn a_reset_to_a_syn_with_data_acknowledges_all_of_it() {
    let _g = fresh();
    inject(Seg::from_peer(9, 1000, 0, SYN, b"0123456789"));
    crate::peer::drain();
    let out = sent();
    assert_eq!(out.len(), 1, "{out:?}");
    assert!(out[0].has(RST) && out[0].has(ACK));
    assert_eq!((out[0].seq, out[0].ack), (0, 1011), "SYN plus ten data bytes");
}
