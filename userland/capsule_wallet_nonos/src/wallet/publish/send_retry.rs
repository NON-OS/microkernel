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

/*
 * A publisher broadcast's answer (sent, below) and its retries (B8, the
 * the rulings on commits 4 and 5): each a read, each taken as an
 * intent::Turn that leaves no press standing, the guard started afresh.
 * After a broadcast settle did not confirm, or once the send ledger says
 * the send followed was dropped, GAS is read again before anything is
 * offered (intent::after_reread): its nonce used, the send is followed as
 * it was; unused, its review comes back for the guarded Sign. Nothing
 * here signs or broadcasts.
 */

use nonos_settle_proto::Gas;

use super::send_drive::EVERY_MS;
use super::sending::Sending;
use crate::wallet::intent::{after_broadcast, after_reread, echo, IntentStep, Turn};
use crate::wallet::state::State;

/// A Turn taken: the guard starts afresh, so a review offered again waits
/// for its own second of focus before Sign acts.
pub fn took(state: &mut State, turn: Turn) -> bool {
    if let Some(s) = state.publisher.sending.as_mut() {
        (s.step, s.pressed) = (turn.step, turn.pressed);
    }
    state.arm = Default::default();
    true
}

/// GAS's answer while the broadcast is unsure, or the send followed is
/// dropped; none read, it is asked again.
pub fn reread(state: &mut State, gas: Option<Gas>, now_ms: u64) -> bool {
    let Some(s) = state.publisher.sending.as_mut() else { return false };
    s.next_ms = now_ms + EVERY_MS;
    let Some(g) = gas else { return false };
    let turn = after_reread(&mut state.broadcast.sent, s.step, s.signed, g.tx_nonce);
    took(state, turn)
}

/// BROADCAST's answer: the hash signed is followed, and an answer naming
/// another is kept apart, said as a warning and never followed.
pub fn sent(s: &mut Sending, answer: Option<[u8; 32]>, now_ms: u64) -> IntentStep {
    (s.next_ms, s.echo) = (now_ms + EVERY_MS, echo(answer, s.signed));
    after_broadcast(answer, s.signed)
}
