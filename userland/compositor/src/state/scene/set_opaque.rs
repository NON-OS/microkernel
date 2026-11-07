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

//! What a present said of its buffer's pixels, kept on the layer it went to,
//! so damage under an opaque layer is not composed (state/hidden.rs).

use super::layer::Layer;
use super::table::SceneTable;

impl SceneTable {
    /// `owner_pid` presented `handle` with every pixel opaque or not. Each of
    /// its layers showing that buffer takes it, in whatever band: the pixels
    /// are the buffer's, and a layer moved between bands shows it in both
    /// for a step (twin.rs). False when none shows it.
    pub fn set_opaque(&mut self, owner_pid: u32, handle: u64, opaque: bool) -> bool {
        let shows =
            |l: &&mut Layer| l.in_use && l.owner_pid == owner_pid && l.surface_handle == handle;
        let mut any = false;
        for l in self.entries.iter_mut().filter(shows) {
            l.opaque = opaque;
            any = true;
        }
        any
    }

    /// `owner_pid` drew in the surfaces its layers `drawn` picks and said
    /// nothing of their pixels (a damage commit): they are no longer taken
    /// as opaque until a present says so again.
    pub fn drew(&mut self, owner_pid: u32, drawn: impl Fn(&Layer) -> bool) {
        let mine = |l: &&mut Layer| l.in_use && l.owner_pid == owner_pid;
        for l in self.entries.iter_mut().filter(mine) {
            if drawn(l) {
                l.opaque = false;
            }
        }
    }
}
