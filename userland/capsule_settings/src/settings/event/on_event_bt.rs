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

//! The keys of the Bluetooth page, which keeps its own: arrows walk the
//! device list, letters scan, pair and forget, digits type a passkey. What
//! each key asks is decided in `crate::bluetooth::panel::act`.

use nonos_app_skeleton::EventOutcome;

use crate::bluetooth::panel::{act, bt_key, Step};
use crate::bluetooth::run;
use crate::settings::state::State;

use super::next_section::{next_section, prev_section};

pub(super) fn on_event_bt(state: &mut State, code: u32) -> EventOutcome {
    let Some(key) = bt_key(code) else { return EventOutcome::Idle };
    match act(&mut state.bt, key) {
        Step::Close => return EventOutcome::Close,
        Step::NextSection => next_section(state),
        Step::PrevSection => prev_section(state),
        Step::Ask(req) => run(&mut state.bt, req),
        Step::Shown => {}
        Step::Nothing => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

/// A click on a device highlights it, as the arrows do; F then forgets it.
pub(super) fn pick_device(state: &mut State, i: usize) -> EventOutcome {
    state.bt.cursor = i;
    EventOutcome::Repaint
}
