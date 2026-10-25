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

//! The receive screen, as presses.

use nonos_app_skeleton::EventOutcome;

use super::etna_click::go;
use crate::wallet::copy::copy_address;
use crate::wallet::screen;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_HOME};

pub(super) fn receive(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Footer(0) => {
            let private = state.acct.receive_private && crate::wallet::shield::open::here();
            let copied = match (&state.shield_ui.nox1, private) {
                (Some(nox1), true) => copy_address(nox1),
                /* The private address is shown as not ready: the public one
                 * is never copied in its place. */
                (None, true) => {
                    state.status = b"the private address is not ready yet";
                    return EventOutcome::Repaint;
                }
                _ => copy_address(&screen::receive_address::address_text(state)),
            };
            state.status = match copied {
                Ok(()) => b"address copied",
                Err(why) => why,
            };
            EventOutcome::Repaint
        }
        Press::Footer(1) if !crate::wallet::chain::current().shield => {
            state.status = b"the shield runs on Sepolia only, switch in Settings";
            EventOutcome::Repaint
        }
        Press::Footer(1) => {
            state.acct.receive_private = !state.acct.receive_private;
            if state.acct.receive_private {
                crate::wallet::shield::open::ensure(state);
            }
            state.scroll = 0;
            EventOutcome::Repaint
        }
        Press::Back => go(state, VIEW_HOME),
        _ => EventOutcome::Idle,
    }
}
