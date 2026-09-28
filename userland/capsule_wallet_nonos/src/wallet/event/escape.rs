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
 * Esc goes back one step: an open panel closes, any other screen returns
 * home, and home stays where it is. It used to close the window, so every
 * "Esc to cancel" on screen was a promise the key could not keep, and a
 * reader backing out of a send lost the wallet window instead.
 */

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::{State, VIEW_HOME, VIEW_SHIELD};

pub fn escape(state: &mut State) -> EventOutcome {
    if state.panel != 0 {
        state.panel = 0;
        return EventOutcome::Repaint;
    }
    if state.view == VIEW_SHIELD {
        return crate::wallet::screen::shield::click::back(state);
    }
    if state.view != VIEW_HOME {
        state.view = VIEW_HOME;
        state.scroll = 0;
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}
