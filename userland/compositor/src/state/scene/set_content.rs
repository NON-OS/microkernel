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

use super::layer::{Content, Layer};
use super::table::SceneTable;

impl SceneTable {
    /// `owner_pid`'s layer showing `on` draws `content` over its own surface
    /// from now on; a handle of 0 draws nothing there. The layer as it was,
    /// or None when `owner_pid` has no layer showing `on`, or when `content`
    /// names a surface another layer shows or draws over its own: one buffer
    /// is never two windows' picture.
    pub fn set_content(&mut self, owner_pid: u32, on: u64, content: Content) -> Option<Layer> {
        let h = content.handle;
        let elsewhere = |l: &Layer| {
            l.in_use && (l.surface_handle == h || (l.content.handle == h && l.surface_handle != on))
        };
        if h != 0 && self.entries.iter().any(elsewhere) {
            return None;
        }
        let mine = |l: &&mut Layer| l.in_use && l.owner_pid == owner_pid && l.surface_handle == on;
        let l = self.entries.iter_mut().find(mine)?;
        let was = *l;
        l.content = if h == 0 { Content::default() } else { content };
        Some(was)
    }

    /// The surface `handle` is gone: no layer draws it over its own any
    /// more. The layers that did, as they were.
    pub fn drop_content(&mut self, handle: u64, out: &mut [Layer]) -> usize {
        let mut n = 0;
        for l in self.entries.iter_mut().filter(|l| l.in_use && handle != 0) {
            if l.content.handle == handle && n < out.len() {
                out[n] = *l;
                n += 1;
                l.content = Content::default();
            }
        }
        n
    }
}
