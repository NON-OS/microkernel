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

use super::layer::Layer;
use super::refused::Refused;
use super::table::SceneTable;

impl SceneTable {
    // A layer named by its owner, band and surface handle. An owner may hold
    // several in one band, one per window, dialog or popup. A resubmit of the
    // same handle moves or resizes that layer alone; with `replaces`, the
    // layer that showed that handle takes the new one (a window that reopened
    // its surface to resize). Either way the layer keeps its place in the
    // stack and the pointer shape its owner set over it. A new handle goes on
    // top of its band. Hands back the layer as it was before, when there was
    // one, so the caller repaints where it stood and lets go of its surface.
    pub fn submit_keyed(
        &mut self,
        layer: Layer,
        replaces: Option<u64>,
    ) -> Result<Option<Layer>, Refused> {
        let layer = Layer { keyed: true, ..layer };
        let same = |l: &Layer, handle: u64| {
            l.in_use
                && l.keyed
                && l.owner_pid == layer.owner_pid
                && l.z == layer.z
                && l.surface_handle == handle
        };
        let mut at = self.entries.iter().position(|l| same(l, layer.surface_handle));
        if let Some(old) = replaces.filter(|&h| h != layer.surface_handle) {
            // The handle being replaced is the layer that moves on; a second
            // layer already showing the new handle would be left a twin.
            if at.is_some() {
                return Err(Refused::NoLayer);
            }
            at = Some(self.entries.iter().position(|l| same(l, old)).ok_or(Refused::NoLayer)?);
        }
        let Some(at) = at else {
            self.place_new(layer)?;
            return Ok(None);
        };
        let was = self.entries[at];
        let (stack, cursor, content) = (was.stack, was.cursor, was.content);
        self.entries[at] = Layer { stack, cursor, content, ..layer };
        Ok(Some(was))
    }
}
