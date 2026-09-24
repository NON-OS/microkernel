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

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::State;

/// Broadcasting is the one thing in this window that cannot be taken back.
///
/// It used to happen on a single keypress. Meanwhile the process manager makes
/// you press twice to end a process and refuses outright to end a core one, so
/// the system asked for confirmation before stopping a program and none at all
/// before spending money. That is the wrong way round.
///
/// The first press arms and says what will happen; the second sends. Anything
/// that changes what would be sent disarms, so a confirmation can never carry
/// over to a transaction the reader was not looking at when they armed it.
pub fn arm_or_send(state: &mut State) -> EventOutcome {
    if !state.tx_ready || state.tx_raw.is_empty() {
        state.status = b"nothing signed to send";
        state.broadcast_armed = false;
        return EventOutcome::Repaint;
    }
    if state.broadcast_armed {
        state.broadcast_armed = false;
        return super::broadcast::broadcast(state);
    }
    state.broadcast_armed = true;
    state.status = b"press b again to send this transaction, or Esc to cancel";
    EventOutcome::Repaint
}

/// Cancel an armed send. Called from every path that changes the view or
/// replaces the signed transaction.
pub fn disarm(state: &mut State) {
    state.broadcast_armed = false;
}
