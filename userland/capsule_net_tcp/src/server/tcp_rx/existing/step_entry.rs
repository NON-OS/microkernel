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
use crate::server::tcp_rx::{rst, transitions};
use crate::state::{Entry, TimerKind};
use crate::tcp::{State, TcpHeader, FLAG_ACK, FLAG_RST};

pub fn step_entry(e: &mut Entry, hdr: &TcpHeader, payload: &[u8], now: u64, accepted: &mut Option<(u32, u32)>, arm: &mut Option<(u32, TimerKind, u64)>) -> RxAction {
    if hdr.has_flag(FLAG_RST) {
        return rst::judge(e, hdr);
    }
    if e.tcb.state == State::SynSent {
        return transitions::handshake::step(e, hdr);
    }
    if e.tcb.state == State::SynReceived && hdr.has_flag(FLAG_ACK) && hdr.ack == e.tcb.send.nxt {
        *accepted = Some((e.parent, e.handle));
        return transitions::syn_received::complete(e, hdr, payload);
    }
    if e.tcb.state.is_closing() {
        let (action, deadline) = transitions::closing::step(e, hdr, payload, now);
        if let Some(d) = deadline {
            // The one deadline a closing step sets is how long its new state may last.
            let kind = if e.tcb.state == State::FinWait2 {
                TimerKind::FinWait2
            } else {
                TimerKind::TimeWait
            };
            *arm = Some((e.handle, kind, d));
        }
        return action;
    }
    if e.tcb.state == State::CloseWait {
        return transitions::close_wait::step(e, hdr, payload);
    }
    if e.tcb.state.accepts_data() {
        return transitions::established::step(e, hdr, payload);
    }
    RxAction::None
}
