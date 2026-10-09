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
use crate::tcp::seq;

/// The largest window a peer can offer without window scaling, which this
/// stack never negotiates: how far behind SND.UNA an old ACK can still be.
const MAX_SND_WND: u32 = 65_535;

/*
 * RFC 5961 5.2: an ACK is acceptable in SND.UNA - MAX.SND.WND =< SEG.ACK
 * =< SND.NXT. One past SND.NXT acknowledges bytes never sent, and one further
 * back than any window explains is not from this connection. A blind sender
 * only has to land its sequence number inside the receive window to have
 * its data taken; it also has to guess this to get past here.
 */
pub fn acceptable(e: &Entry, ack: u32) -> bool {
    seq::leq(ack, e.tcb.send.nxt) && seq::leq(e.tcb.send.una.wrapping_sub(MAX_SND_WND), ack)
}
