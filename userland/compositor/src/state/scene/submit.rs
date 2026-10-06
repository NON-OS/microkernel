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

use super::layer::{Layer, MAX_LAYERS, MAX_LAYERS_PER_OWNER};
use super::table::SceneTable;

impl SceneTable {
    // A layer is its owner's in one z band: a process holds at most one layer
    // per band, and a resubmit in that band (a move, a resize, a new surface)
    // replaces it and keeps its place in the stack; only a raise changes that.
    // A layer submitted for the first time goes on top of its band, where the
    // window manager stacks a window it has just opened.
    //
    // It was one layer per process, whatever its band, so the desktop shell
    // could not hold its desktop under the windows and its menus and dock over
    // them: everything it drew sat in one band, under every window.
    //
    // A full table, or an owner already holding its cap of bands, is the one
    // failure, so the error carries nothing more.
    #[allow(clippy::result_unit_err)]
    pub fn submit(&mut self, layer: Layer) -> Result<(), ()> {
        for slot in self.entries.iter_mut() {
            if slot.in_use && slot.owner_pid == layer.owner_pid && slot.z == layer.z {
                let stack = slot.stack;
                *slot = Layer { stack, ..layer };
                return Ok(());
            }
        }
        if self.count >= MAX_LAYERS {
            return Err(());
        }
        let held = self.entries.iter().filter(|l| l.in_use && l.owner_pid == layer.owner_pid);
        if held.count() >= MAX_LAYERS_PER_OWNER {
            return Err(());
        }
        let stack = self.take_stack();
        for slot in self.entries.iter_mut() {
            if !slot.in_use {
                *slot = Layer { stack, ..layer };
                self.count += 1;
                return Ok(());
            }
        }
        Err(())
    }
}
