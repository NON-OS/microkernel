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

use crate::server::tcp_rx::action::RxAction;
use crate::server::tcp_rx::transitions::established::{ack_range, handle_ack, handle_dup_ack, handle_fin, handle_payload, reply_ack};
use crate::state::Entry;
use crate::tcp::{seq, TcpHeader, FLAG_ACK};

pub fn step(e: &mut Entry, hdr: &TcpHeader, payload: &[u8]) -> RxAction {
    if !seq::acceptable(hdr.seq, payload.len() as u32, e.tcb.recv.nxt, e.tcb.recv.wnd) {
        return reply_ack::reply_ack(e);
    }
    // Every segment of a synchronized connection carries an ACK; one without
    // is dropped, and one whose ACK this side could not have earned is
    // dropped and answered (RFC 9293 3.10.7.4).
    if !hdr.has_flag(FLAG_ACK) {
        return RxAction::None;
    }
    if !ack_range::acceptable(e, hdr.ack) {
        return reply_ack::reply_ack(e);
    }
    // A duplicate ACK leaves the window as it was (RFC 5681 section 2); one
    // that moved it is a window update, judged against the window before it.
    let window_before = e.tcb.send.wnd;
    if !handle_ack::handle_ack(e, hdr) && hdr.window == window_before {
        handle_dup_ack::handle_dup_ack(e, hdr, payload);
    }
    handle_payload::handle_payload(e, hdr, payload);
    handle_fin::handle_fin(e, hdr);
    reply_ack::reply_ack(e)
}
