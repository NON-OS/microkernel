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

use alloc::vec::Vec;

use crate::server::tcp_rx::action::RxAction;
use crate::state::Entry;
use crate::tcp::{State, TcpHeader, FLAG_ACK};

pub fn in_window(e: &Entry, seq: u32) -> bool {
    let wnd = (e.tcb.recv.wnd as u32).max(1);
    seq.wrapping_sub(e.tcb.recv.nxt) < wnd
}

/*
 * SYN-SENT has no receive sequence yet, so a reset there is believed only
 * when it acknowledges our SYN (RFC 9293 3.10.7.3); judged against an RCV.NXT
 * still at zero, any reset with a small sequence number ended the attempt.
 * In every other state RFC 5961 3.2 applies: a reset exactly at RCV.NXT
 * ends the connection, one elsewhere in the window draws a challenge ACK
 * that a real peer answers with an exact reset, and one outside it is
 * dropped. A blind sender then needs one sequence number, not a window's
 * worth of them.
 */
pub fn judge(e: &Entry, hdr: &TcpHeader) -> RxAction {
    if e.tcb.state == State::SynSent {
        if hdr.has_flag(FLAG_ACK) && hdr.ack == e.tcb.send.nxt {
            return RxAction::Reap(e.handle);
        }
        return RxAction::None;
    }
    if hdr.seq == e.tcb.recv.nxt {
        return RxAction::Reap(e.handle);
    }
    if in_window(e, hdr.seq) {
        return RxAction::Reply(e.tcb, FLAG_ACK, Vec::new());
    }
    RxAction::None
}
