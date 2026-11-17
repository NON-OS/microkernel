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

//! The pointer over whichever menu is open: a move repaints only when it
//! changes the row lit, and a press starts the pointer afresh, since every
//! press opens or closes a menu.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};
use nonos_toolkit::widgets::Pointer;

use super::app::Editor;
use super::menu_view::bounds;
use super::menubar::dropdown_lit;
use super::ribbon::{ribbon_drop_lit, ribbon_state};
use super::sb_menu::menu_lit;

impl Editor {
    /// The row the pointer lights on the open menu, if any.
    fn menu_row_lit(&self) -> Option<usize> {
        let (b, ptr) = (bounds(self.last_w, self.last_h), &self.menu_ptr);
        if let Some(menu) = &self.menu {
            return menu_lit(menu, b, ptr);
        }
        if let Some(open) = self.mb_open {
            return dropdown_lit(&self.mb_layout, open, b, ptr);
        }
        let pill = self.rb_open?;
        ribbon_drop_lit(&self.rb_layout, pill, &ribbon_state(self.doc_ref()), b, ptr)
    }

    /// Feeds a move to the open menu: a repaint when it lights another row,
    /// else None so the move goes on as before.
    pub(super) fn menu_pointer(&mut self, event: &InputEvent) -> Option<EventOutcome> {
        match event.kind {
            InputKind::ButtonDown => self.menu_ptr = Pointer::new(),
            InputKind::PointerAbs => {
                let before = self.menu_row_lit();
                self.menu_ptr.moved(event.x, event.y);
                if self.menu_row_lit() != before {
                    return Some(EventOutcome::Repaint);
                }
            }
            _ => {}
        }
        None
    }
}
