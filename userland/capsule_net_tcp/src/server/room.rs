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
//! Telling the peer about room a reader made.

use crate::state::Entry;
use crate::tcp::persist::worth_announcing;
use crate::tcp::{State, FLAG_ACK, MSS, RWND_MAX};

/// After a read, announce the receive window when it grew by enough since it
/// was last advertised. A peer that saw it closed sends nothing until told,
/// and before this nothing told it until the peer's own persist probe.
pub fn announce(e: &mut Entry) {
    if !matches!(e.tcb.state, State::Established | State::FinWait1 | State::FinWait2) {
        return;
    }
    let room = e.rwnd();
    if !worth_announcing(e.tcb.recv.wnd, room, MSS, RWND_MAX) {
        return;
    }
    e.tcb.recv.wnd = room;
    let _ = crate::server::tcp_tx::send(e.tcb, FLAG_ACK, &[]);
}
