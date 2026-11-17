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

//! The short .anyone name notice: a `Warn` toast across the top of the
//! page while the page shown was reached by a short name. It is drawn after
//! every page paint, so a scroll or a partial repaint never leaves the page
//! without it.

use nonos_app_skeleton::PaintBuffer;

use crate::browser::short_name::{notice_for, notice_panel};
use crate::browser::state::{State, View};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if state.view != View::Page {
        return;
    }
    if let Some(notice) = state.ui.history.current().and_then(notice_for) {
        let (toast, at) = notice_panel(notice, fb.width);
        toast.draw(fb, at);
    }
}
