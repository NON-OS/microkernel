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
use crate::server::tcp_rx::transitions::established::{ack_range, handle_ack, reply_ack};
use crate::state::Entry;
use crate::tcp::{seq, TcpHeader, FLAG_ACK, FLAG_FIN};

/*
 * CLOSE-WAIT (RFC 9293 3.10.7.4): the peer has sent its FIN, this side has
 * not. The peer's ACKs still acknowledge this side's data and move its
 * window, so they are taken; ignoring them left acknowledged data queued for
 * retransmission until the retry limit tore the connection down. A segment
 * outside the window, the peer's FIN again among them when this side's ACK
 * of it was lost, is answered with an ACK. Data after the FIN is not taken.
 */
pub fn step(e: &mut Entry, hdr: &TcpHeader, payload: &[u8]) -> RxAction {
    let fin = u32::from(hdr.has_flag(FLAG_FIN));
    if !seq::acceptable(hdr.seq, payload.len() as u32 + fin, e.tcb.recv.nxt, e.tcb.recv.wnd) {
        return reply_ack::reply_ack(e);
    }
    if !hdr.has_flag(FLAG_ACK) {
        return RxAction::None;
    }
    if !ack_range::acceptable(e, hdr.ack) {
        return reply_ack::reply_ack(e);
    }
    handle_ack::handle_ack(e, hdr);
    RxAction::None
}
