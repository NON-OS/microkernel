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
use crate::state::Entry;
use crate::tcp::{State, TcpHeader, FLAG_FIN};

/*
 * RFC 9293 3.10.7.4, SYN-RECEIVED: the ACK of our SYN moves SND.UNA past it
 * and gives the first send window. Without that, SND.UNA stayed where the
 * listener left it, the bytes in flight read as the whole ISS, and the new
 * connection never had a window to send in. Data and a FIN may ride on the
 * same ACK; they are taken as on any established segment, not dropped.
 */
pub fn complete(e: &mut Entry, hdr: &TcpHeader, payload: &[u8]) -> RxAction {
    e.tcb.state = State::Established;
    e.tcb.send.una = hdr.ack;
    e.tcb.send.wnd = hdr.window;
    e.tcb.send.wl1 = hdr.seq;
    e.tcb.send.wl2 = hdr.ack;
    if payload.is_empty() && !hdr.has_flag(FLAG_FIN) {
        return RxAction::None;
    }
    super::established::step(e, hdr, payload)
}
