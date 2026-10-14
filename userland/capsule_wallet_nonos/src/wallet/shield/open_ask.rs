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

//! Asking the service whether it holds this account, and opening it.

use super::*;

pub fn open(state: &mut State) {
    if state.shield_ui.waiting.is_some() {
        return;
    }
    if !here() {
        state.shield_ui.failure = Some(String::from(ELSEWHERE));
        return;
    }
    if !state.acct.address_ready {
        state.shield_ui.failure = Some(String::from(NO_ACCOUNT));
        return;
    }
    let account = address_hex(state);
    /* Asked before the words are: a service that does not answer never gets them. */
    let mut running = None;
    match held(state, &account, &mut running) {
        Err(NO_ANSWER) => return later(state),
        Err(why) => {
            state.shield_ui.failure = Some(String::from(why));
            return;
        }
        Ok(true) => {
            answered(state);
            state.shield_ui.opened = true;
            /* A job still running for this wallet, as after the window
             * restarted mid-proof, is waited on, not started over. The pool
             * is read again only when the last read is a sync interval old,
             * so going in and out of Shield never holds every button grey. */
            let resumed =
                running.as_deref().is_some_and(|name| super::super::job::resume(state, name));
            if !resumed && super::super::follow::sync_stale(&state.shield_ui, mk_uptime_ms()) {
                state.shield_ui.link.last_sync_ms = mk_uptime_ms();
                super::super::job::start(state, OP_SYNC, &[]);
            }
            return;
        }
        Ok(false) => answered(state),
    }
    from_words(state, &account);
}
