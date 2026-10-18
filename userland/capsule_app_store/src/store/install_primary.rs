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

//! Enter does the next sensible thing for the selected listing: install it,
//! wait while it installs, or open it once it has.

use super::channel_held::{queued, reads_channels, route, Route};
use super::install::{ask, open, remove, Asked};
use super::meter_call::funded;
use super::next_step::{on_enter, Enter};
use super::purchase::{names_of, post};
use super::purchase_said::ordered;
use super::state::State;

/// What Enter means for the selected listing, as far as it has got.
pub fn primary(state: &mut State) -> Option<Asked> {
    let (progress, removing) = state.current().map(|l| (l.progress, l.removing))?;
    match on_enter(progress, removing) {
        Enter::Open => Some(open(state)),
        Enter::Wait => None,
        Enter::Remove => Some(remove(state)),
        // The pane says why; asking again would only fail the same way.
        Enter::Cannot => Some(Asked::Cannot),
        Enter::Install => {
            let l = state.current()?;
            let bit = reads_channels(l).then(|| funded(&names_of(l))).flatten();
            Some(match route(l, bit) {
                Route::Install => install(state),
                Route::Post => buy(state),
            })
        }
    }
}

/// A sold listing is bought in the wallet, which installs it after; one
/// whose channel is already open and funded installs (channel_held).
fn buy(state: &mut State) -> Asked {
    let asked = post(state);
    if asked == Asked::InWallet && state.current().is_some_and(ordered) {
        state.orders.posted = state.current().map(names_of);
        state.orders.order = None;
    }
    asked
}

fn install(state: &mut State) -> Asked {
    let asked = ask(state);
    if let Some(l) = state.current_mut() {
        queued(l, asked);
    }
    asked
}
