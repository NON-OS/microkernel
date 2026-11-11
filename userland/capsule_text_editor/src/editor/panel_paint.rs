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

//! Paint an open panel over the shell as the shared card of rows, laying its
//! rows out in the panel's controls as it goes.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::app::Editor;
use super::layout::titlebar_h;
use super::panel::{panel_note, panel_title, Panel};
use super::panel_view::PanelView;

impl Editor {
    pub(super) fn paint_panel(&mut self, fb: &mut PaintBuffer, panel: Panel) {
        let owned = self.panel_rows(panel);
        let rows: Vec<(&str, &str)> = owned.iter().map(|(l, v)| (*l, v.as_str())).collect();
        let view = PanelView {
            title: panel_title(panel),
            note: panel_note(panel),
            rows: &rows,
            picks: panel == Panel::Special,
        };
        let at = view.rect(self.last_w, self.last_h, titlebar_h());
        view.paint(fb, at, &mut self.panel_ctl);
    }
}
