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

//! One pass of the serve loop, against the shim's inbox. The driver used to
//! sleep holding the claim, a minute at a time, serving nothing. It waits in
//! its receive now, for at most `WAIT_MS`, and answers what comes.

use nonos_libc::bringup::recv_turn::RECV_PARK_MS;
use nonos_libc::{idles, replies, send, waits};

use super::frames::{fields, frame, status_of};
use super::model::{served, Served, COMPOSITOR};
use crate::serve::WAIT_MS;
use crate::serve::{turn, Reply, E_INVAL, OP_FLUSH, OP_GET_PRIMARY_SURFACE, REPLY_LEN, RX_LEN};

fn one_turn(s: &mut Served) {
    let (mut rx, mut tx): ([u8; RX_LEN], Reply) = ([0; RX_LEN], [0; REPLY_LEN]);
    turn(&mut s.d, &mut rx, &mut tx);
}

#[test]
fn a_turn_waits_in_its_receive_with_a_deadline_and_sleeps_nowhere() {
    let mut s = served(64, 48);
    one_turn(&mut s);
    one_turn(&mut s);
    assert_eq!(waits(), [WAIT_MS, WAIT_MS], "each receive is asked for a deadline");
    assert!(waits().iter().all(|&ms| ms != 0), "a receive asked to wait 0 waits for ever");
    assert_eq!(idles(), [], "a turn slept instead of waiting in its receive");
    assert_eq!(replies(), [], "answered a request nobody sent");
}

#[test]
fn a_receive_that_fails_at_once_parks_rather_than_spins() {
    let mut s = served(64, 48);
    send(Err(-2));
    one_turn(&mut s);
    assert_eq!(idles(), [RECV_PARK_MS], "a receive that failed at once went round at once");
}

#[test]
fn a_request_is_answered_to_its_sender_and_one_from_the_kernel_is_not() {
    let mut s = served(64, 48);
    send(Ok((0, &frame(OP_GET_PRIMARY_SURFACE, 4, &[]))));
    send(Ok((COMPOSITOR, &frame(OP_GET_PRIMARY_SURFACE, 5, &[]))));
    one_turn(&mut s);
    one_turn(&mut s);
    let r = replies();
    assert_eq!((r.len(), r[0].0, status_of(&r[0].1)), (1, COMPOSITOR, 0), "not one reply");
    assert_eq!(r[0].1[12..16], 5u32.to_le_bytes(), "answered under another request id");
}

#[test]
fn a_frame_longer_than_the_buffer_is_refused_not_read_cut() {
    // The header names what fits, so the frame as cut parses whole.
    let mut s = served(64, 48);
    let mut f = frame(OP_FLUSH, 6, &fields(&[1, 0, 0, 1, 1]));
    f.resize(RX_LEN + 64, 0);
    f[16..20].copy_from_slice(&(RX_LEN as u32 - 20).to_le_bytes());
    send(Ok((COMPOSITOR, &f)));
    one_turn(&mut s);
    assert_eq!(status_of(&replies()[0].1), E_INVAL, "a cut frame was read as a request");
}
