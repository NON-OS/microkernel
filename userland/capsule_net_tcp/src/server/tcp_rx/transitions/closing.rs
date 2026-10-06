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
use crate::server::tcp_rx::transitions::established::handle_payload::handle_payload;
use crate::state::Entry;
use crate::tcp::{seq, State, TcpHeader, FLAG_ACK, FLAG_FIN};

pub fn step(e: &mut Entry, hdr: &TcpHeader, payload: &[u8], now_ms: u64) -> (RxAction, Option<u64>) {
    let fin = u32::from(hdr.has_flag(FLAG_FIN));
    let rwnd = e.rwnd();
    e.tcb.recv.wnd = rwnd;
    if !seq::acceptable(hdr.seq, payload.len() as u32 + fin, e.tcb.recv.nxt, e.tcb.recv.wnd) {
        return (RxAction::Reply(e.tcb, FLAG_ACK, Vec::new()), None);
    }
    let acks_fin = hdr.has_flag(FLAG_ACK) && hdr.ack == e.tcb.send.nxt;
    if acks_fin {
        e.tcb.send.una = hdr.ack;
    }
    /*
     * FIN-WAIT-1 and FIN-WAIT-2 have closed our side only. The peer's data
     * still arrives, and its FIN is the sequence number after the last byte
     * of it (RFC 9293 3.10.7.4), taken once everything before it has been.
     * Taking it at SEG.SEQ + 1 acknowledged data that had been dropped.
     */
    let receiving = matches!(e.tcb.state, State::FinWait1 | State::FinWait2);
    let before = e.tcb.recv.nxt;
    if receiving {
        handle_payload(e, hdr, payload);
        e.tcb.recv.wnd = e.rwnd();
    }
    let took_data = e.tcb.recv.nxt != before;
    let has_fin = fin == 1
        && (!receiving
            || (e.reasm.is_empty()
                && hdr.seq.wrapping_add(payload.len() as u32) == e.tcb.recv.nxt));
    match e.tcb.state {
        State::FinWait1 if acks_fin && has_fin => to_timewait(e, now_ms),
        State::FinWait1 if acks_fin => {
            e.tcb.state = State::FinWait2;
            (ack_if(took_data, e), Some(now_ms.saturating_add(crate::tcp::FIN_WAIT_2_MS)))
        }
        State::FinWait1 if has_fin => {
            e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(1);
            e.tcb.state = State::Closing;
            (RxAction::Reply(e.tcb, FLAG_ACK, Vec::new()), None)
        }
        State::FinWait2 if has_fin => to_timewait(e, now_ms),
        State::Closing if acks_fin => {
            e.tcb.state = State::TimeWait;
            (RxAction::None, Some(now_ms + crate::tcp::msl_2_ms()))
        }
        State::LastAck if acks_fin => (RxAction::Reap(e.handle), None),
        // A FIN seen again: acknowledge it again and restart the wait.
        State::TimeWait if has_fin => {
            (RxAction::Reply(e.tcb, FLAG_ACK, Vec::new()), Some(now_ms + crate::tcp::msl_2_ms()))
        }
        _ => (ack_if(took_data, e), None),
    }
}

fn to_timewait(e: &mut Entry, now_ms: u64) -> (RxAction, Option<u64>) {
    e.tcb.recv.nxt = e.tcb.recv.nxt.wrapping_add(1);
    e.tcb.state = State::TimeWait;
    (RxAction::Reply(e.tcb, FLAG_ACK, Vec::new()), Some(now_ms + crate::tcp::msl_2_ms()))
}

fn ack_if(took_data: bool, e: &Entry) -> RxAction {
    if took_data {
        RxAction::Reply(e.tcb, FLAG_ACK, Vec::new())
    } else {
        RxAction::None
    }
}
