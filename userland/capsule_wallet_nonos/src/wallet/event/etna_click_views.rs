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

//! The first screen and the home screen, as presses.

use nonos_app_skeleton::EventOutcome;

use super::etna_click::go;
use crate::wallet::screen;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::{
    State, VIEW_ACCOUNTS, VIEW_IMPORT, VIEW_NOX, VIEW_RECEIVE, VIEW_RECOVER, VIEW_SETTINGS,
    VIEW_SWAP,
};

/// The first screen: make a wallet, import a key, or restore from words.
pub(super) fn welcome(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Footer(0) => match super::may_replace(state) {
            Ok(()) => super::custody::generate::generate(state),
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

pub(super) fn home(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Send => screen::pay::click::open(state),
        Press::Receive => {
            /* Private by default only where the shield runs. */
            state.acct.receive_private = crate::wallet::chain::current().shield;
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
