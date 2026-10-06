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

use crate::state::{Entry, RX_DEPTH};
use crate::tcp::{seq, TcpHeader, MSS};

pub fn handle_payload(e: &mut Entry, hdr: &TcpHeader, payload: &[u8]) {
    let (at, payload) = trim_taken(e.tcb.recv.nxt, hdr.seq, payload);
    if payload.is_empty() {
        return;
    }
    if at == e.tcb.recv.nxt && e.rx.len() < RX_DEPTH && e.push_rx(payload) {
        e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(payload.len() as u32);
        take_reassembled(e);
    } else if seq::gt(at, e.tcb.recv.nxt) {
        e.reasm.insert(at, payload.to_vec());
    }
}

/*
 * A retransmission the peer packed again can start before RCV.NXT and run
 * past it. The bytes already taken are cut off and the rest is new data at
 * RCV.NXT (RFC 9293 3.10.7.4); dropping the whole segment lost those bytes
 * until the peer happened to send them again on their own.
 */
fn trim_taken(rcv_nxt: u32, at: u32, payload: &[u8]) -> (u32, &[u8]) {
    if !seq::lt(at, rcv_nxt) {
        return (at, payload);
    }
    let behind = rcv_nxt.wrapping_sub(at) as usize;
    (rcv_nxt, payload.get(behind..).unwrap_or(&[]))
}

/*
 * What the reassembly buffer held behind a gap drains as one block once the
 * gap fills, up to every held segment joined. It is queued for the reader in
 * pieces no larger than a segment, so the window, which counts the queue in
 * segments, stays true to the bytes held, and a block that does not fit goes
 * back where it was instead of being dropped.
 */
fn take_reassembled(e: &mut Entry) {
    let more = e.reasm.drain_contiguous(e.tcb.recv.nxt);
    let mut taken = 0usize;
    for piece in more.chunks(MSS) {
        if e.rx.len() >= RX_DEPTH || !e.push_rx(piece) {
            break;
        }
        taken += piece.len();
    }
    e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(taken as u32);
    if taken < more.len() {
        e.reasm.insert(e.tcb.recv.nxt, more[taken..].to_vec());
    }
}
