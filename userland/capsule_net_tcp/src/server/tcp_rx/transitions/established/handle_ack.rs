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

use crate::state::Entry;
use crate::tcp::{seq, TcpHeader, FLAG_ACK};

/// Take the segment's ACK. True when it acknowledged new data.
pub fn handle_ack(e: &mut Entry, hdr: &TcpHeader) -> bool {
    if !hdr.has_flag(FLAG_ACK) || !seq::leq(hdr.ack, e.tcb.send.nxt) {
        return false;
    }
    let new = seq::gt(hdr.ack, e.tcb.send.una);
    if new {
        if let Some(oldest) = e.retx.oldest_mut() {
            if oldest.xmits == 1 {
                let r = crate::clock::now_ms().saturating_sub(oldest.sent_ms).min(crate::tcp::RTO_MAX_MS as u64) as u32;
                e.rtt.on_sample(r);
            }
        }
        e.tcb.send.una = hdr.ack;
        e.retx.ack(hdr.ack);
        e.cc.on_new_ack();
    }
    /*
     * SND.UNA =< SEG.ACK =< SND.NXT updates the window (RFC 9293 3.10.7.4),
     * an ACK of nothing new included: that is how a receiver that closed its
     * window opens it again. Taking the window only with new data left it
     * shut, and what was queued behind it was never sent.
     */
    let mut opened = false;
    if seq::leq(e.tcb.send.una, hdr.ack)
        && crate::tcp::window::should_update(e.tcb.send.wl1, e.tcb.send.wl2, hdr.seq, e.tcb.send.una, hdr.ack)
    {
        opened = hdr.window > e.tcb.send.wnd;
        e.tcb.send.wnd = hdr.window;
        // The peer answered, a probe or anything else: it is still there.
        e.persist.unanswered = 0;
        e.tcb.send.wl1 = hdr.seq;
        e.tcb.send.wl2 = hdr.ack;
    }
    if new || opened {
        crate::server::sender::drain_send(e);
    }
    new
}
