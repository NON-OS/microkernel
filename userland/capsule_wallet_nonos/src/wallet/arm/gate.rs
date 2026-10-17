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
 * The guard at the press and each paint (moves.rs, look.rs, tick.rs): a
 * press that moves value or signs acts only once armed, focus asked of
 * the window manager again at the press; held back, it says so on the
 * status line and nothing else happens. The consent keeps its own guard.
 */

use nonos_app_skeleton::EventOutcome;

use super::look::{painted, pressed};
use super::moves::{moves, Shown};
use super::on_show::on_show;
use super::shows::greys;
use crate::wallet::order::guard::poll::{focused, guarding, now};
use crate::wallet::order::terms::arming::buy_armed;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::State;

const HELD_BACK: &[u8] = b"held back: this acts once the wallet has held focus for a second";

pub(super) fn own(state: &State) -> Option<Shown> {
    on_show(state).filter(|s| *s != Shown::Consent)
}

/// `press` (None: Enter) held back, or None when it may act.
pub fn held_back(state: &mut State, press: Option<Press>) -> Option<EventOutcome> {
    let shown = on_show(state).filter(|s| moves(*s, press))?;
    let (focused, now) = (focused(), now());
    let armed = match shown {
        Shown::Consent => {
            state.meter.purchase.as_deref().is_some_and(|p| buy_armed(p.guard, focused, now))
        }
        s => pressed(state.arm, Some(s), focused, now),
    };
    if armed {
        return None;
    }
    state.status = HELD_BACK;
    Some(EventOutcome::Repaint)
}

/// Whether `press` is drawn greyed: by the fn the press asks (shows.rs).
pub fn greyed(state: &State, press: Press) -> bool {
    greys(&state.arm, own(state), Some(press), now())
}

/// The screen was just painted whole.
pub fn drawn(state: &mut State) {
    state.arm = painted(state.arm, own(state));
}

/// Whether a guarded screen shows, so focus is looked at every short tick.
pub fn looking(state: &State) -> bool {
    guarding(state) || on_show(state).is_some()
}
