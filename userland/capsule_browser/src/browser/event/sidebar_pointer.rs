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

use nonos_app_skeleton::{feed, InputEvent, InputKind};
use nonos_toolkit::widgets::Step;

use crate::browser::omnibox::Change;
use crate::browser::state::State;

/* Pointer motion and a button coming up, fed to the sidebar's rows in
 * window coordinates; off the sidebar the pointer has left them. The
 * sidebar is drawn only with the whole window, so a change of what is lit
 * or pressed redraws that. */
pub(super) fn sidebar_pointer(state: &mut State, event: InputEvent) {
    let over = state.layout.sidebar.contains(event.x, event.y);
    let side = &mut state.ui.side;
    let step = match (event.kind, over) {
        (InputKind::ButtonUp, _) => side.released(),
        (_, true) => feed(side, &event),
        (_, false) => side.left(),
    };
    if step == Step::Repaint {
        state.mark(Change::Full);
    }
}
