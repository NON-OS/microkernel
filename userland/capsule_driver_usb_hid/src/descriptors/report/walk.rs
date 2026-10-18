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

//! The walk's state between items: the globals, the usages of the next main
//! item, the open collections, how far each input report has got, and the
//! layout so far.

use super::cursors::Cursors;
use super::global_state::GlobalState;
use super::item::{Item, GLOBAL, LOCAL, MAIN};
use super::layout::Layout;
use super::limits::Refused;
use super::locals::Locals;

#[derive(Default)]
pub struct Walk {
    pub g: GlobalState,
    pub l: Locals,
    pub depth: u8,
    /// The usage of the application (top level) collection open now, and
    /// how many have opened.
    pub app: u32,
    pub apps: u16,
    pub cursors: Cursors,
    pub layout: Layout,
}

impl Walk {
    pub fn take(&mut self, item: Item) -> Result<(), Refused> {
        match item.kind {
            GLOBAL => self.g.take(item),
            LOCAL => {
                self.l.take(item, self.g.now.page);
                Ok(())
            }
            MAIN => {
                let taken = self.main(item);
                // Local items last until the next main item.
                self.l = Locals::default();
                taken
            }
            _ => Ok(()),
        }
    }
}
