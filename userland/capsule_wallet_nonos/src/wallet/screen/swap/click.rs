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
 * Presses on the Swap screen, what its one button says, and the banner it
 * shows. Choosing a token already on the other side swaps the two, so a
 * pair can never be one token traded for itself. Nothing signs a swap in
 * this build, and the banner says so once an amount is waiting for a price.
 */

use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_HOME};
use crate::wallet::swap::token;

const UNPRICED: &str = "No liquidity pool is connected to this build yet, so this trade \
     cannot be priced or sent. Nothing leaves this machine.";

pub fn failure(state: &State) -> Option<&'static str> {
    (state.swap_in > 0 && !state.swap_quote.ready && !state.swap_note_hidden).then_some(UNPRICED)
}

pub fn action(state: &State) -> &'static str {
    if !state.swap_quote.ready {
        "Enter an amount"
    } else if !token(state.swap_from).is_native() && state.swap_step == 0 {
        "Approve"
    } else {
        "Swap"
    }
}

fn set(state: &mut State, pay: bool, i: u8) {
    let (mine, other) =
        if pay { (state.swap_from, state.swap_to) } else { (state.swap_to, state.swap_from) };
    let other = if other == i { mine } else { other };
    if pay {
        (state.swap_from, state.swap_to) = (i, other);
    } else {
        (state.swap_to, state.swap_from) = (i, other);
    }
    state.swap_step = 0;
    crate::wallet::event::swap_quote::refresh(state);
}

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Back => {
            state.view = VIEW_HOME;
            state.scroll = 0;
        }
        Press::Dismiss => state.swap_note_hidden = true,
        Press::Asset(i) => set(state, true, i),
        Press::Pick(i) => set(state, false, i),
        Press::Footer(0) => state.status = b"signing a swap is not wired in this build",
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
