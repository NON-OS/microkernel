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

/* Pointer motion and a button coming up, fed to the toolbar's controls in
 * the toolbar's own coordinates; off the toolbar the pointer has left them.
 * A change of what is lit or pressed redraws the toolbar. */
pub(super) fn toolbar_pointer(state: &mut State, event: InputEvent) {
    let at = super::local::local(event, state.layout.toolbar);
    let bar = &mut state.ui.bar;
    let step = match (event.kind, at) {
        (InputKind::ButtonUp, _) => bar.released(),
        (_, Some(at)) => feed(bar, &at),
        (_, None) => bar.left(),
    };
    if step == Step::Repaint {
        state.mark(Change::Toolbar);
    }
}
