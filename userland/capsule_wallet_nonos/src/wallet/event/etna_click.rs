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

//! A press, sent to the screen it was drawn on.

use nonos_app_skeleton::clients::clipboard::clipboard_copy;
use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen;
use crate::wallet::screen::hits::{at, Press};
use crate::wallet::state::{
    State, VIEW_ACCOUNTS, VIEW_EXPORT, VIEW_HOME, VIEW_IMPORT, VIEW_NOX, VIEW_RECEIVE,
    VIEW_RECOVER, VIEW_SEND, VIEW_SETTINGS, VIEW_SHIELD, VIEW_SWAP,
};

fn go(state: &mut State, view: u8) -> EventOutcome {
    state.view = view;
    state.scroll = 0;
    state.failure = None;
    EventOutcome::Repaint
}

/// The first screen: make a wallet, import a key, or restore from words.
fn welcome(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Footer(0) => match super::may_replace(state) {
            Ok(()) => super::generate::generate(state),
            Err(why) => {
                state.failure = Some(why);
                EventOutcome::Repaint
            }
        },
        Press::Dismiss => {
            state.failure = None;
            EventOutcome::Repaint
        }
        Press::Footer(1) => screen::custody::click::open(state, VIEW_IMPORT),
        Press::Footer(2) => screen::custody::click::open(state, VIEW_RECOVER),
        _ => EventOutcome::Idle,
    }
}

fn home(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Send => screen::pay::click::open(state),
        Press::Receive => {
            /* Private by default only where the shield runs. */
            state.receive_private = crate::wallet::chain::current().shield;
            crate::wallet::shield::open::ensure(state);
            go(state, VIEW_RECEIVE)
        }
        Press::Swap => go(state, VIEW_SWAP),
        Press::Shield => screen::shield::click::open(state),
        Press::Row(0) => go(state, VIEW_NOX),
        Press::Accounts => {
            crate::wallet::accounts::ensure_root(state);
            go(state, VIEW_ACCOUNTS)
        }
        Press::Settings | Press::Row(1) => go(state, VIEW_SETTINGS),
        Press::Network(_) => go(state, VIEW_SETTINGS),
        _ => EventOutcome::Idle,
    }
}

fn receive(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Footer(0) => {
            let private = state.receive_private && crate::wallet::shield::open::here();
            let copied = match (&state.shield_ui.nox1, private) {
                (Some(nox1), true) => clipboard_copy(nox1.as_bytes()),
                /* The private address is shown as not ready: the public one
                 * is never copied in its place. */
                (None, true) => {
                    state.status = b"the private address is not ready yet";
                    return EventOutcome::Repaint;
                }
                _ => clipboard_copy(screen::receive_address::address_text(state).as_bytes()),
            };
            state.status =
                if copied.is_ok() { b"address copied" } else { b"the clipboard did not take it" };
            EventOutcome::Repaint
        }
        Press::Footer(1) if !crate::wallet::chain::current().shield => {
            state.status = b"the shield runs on Sepolia only, switch in Settings";
            EventOutcome::Repaint
        }
        Press::Footer(1) => {
            state.receive_private = !state.receive_private;
            if state.receive_private {
                crate::wallet::shield::open::ensure(state);
            }
            state.scroll = 0;
            EventOutcome::Repaint
        }
        Press::Back => go(state, VIEW_HOME),
        _ => EventOutcome::Idle,
    }
}

pub fn etna_click(state: &mut State, x: u32, y: u32) -> EventOutcome {
    let Some(press) = at(x, y) else {
        return EventOutcome::Idle;
    };
    if state.locked {
        if press == Press::Footer(0) {
            return super::lock::unlock(state);
        }
        return EventOutcome::Idle;
    }
    if state.backup_active {
        if press != Press::Footer(0) {
            return EventOutcome::Idle;
        }
        let _ = super::backup::confirm_backup(state);
        return go(state, VIEW_HOME);
    }
    match state.view {
        VIEW_IMPORT | VIEW_RECOVER | VIEW_EXPORT => screen::custody::click::click(state, press),
        VIEW_SETTINGS => screen::settings::click::click(state, press),
        VIEW_ACCOUNTS if state.address_ready => screen::accounts::click::click(state, press),
        _ if !state.address_ready => welcome(state, press),
        VIEW_SHIELD => screen::shield::click::click(state, press),
        VIEW_SWAP => screen::swap::click::click(state, press),
        VIEW_SEND => screen::pay::click::click(state, press),
        VIEW_NOX => screen::stake::click::click(state, press),
        VIEW_RECEIVE => receive(state, press),
        _ => home(state, press),
    }
}
