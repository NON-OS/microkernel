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

/* The position rule lives with the painters; it is compiled here so the
 * render harness, which has no paint module, hit-tests the same way. */
#[path = "../../paint/screen_y.rs"]
mod screen_y;

pub use screen_y::frag_screen_y;

use super::boxmodel::{BoxDocument, Fragment};

/* Hit tests in viewport space: (x, y) with y = 0 at the first page row and
 * the page scrolled by `scroll`, so a fixed header or a stuck sticky bar is
 * found where it is painted rather than where it sits in the document.
 * Fragments paint front to back, so the last one hit wins. */
impl BoxDocument {
    pub fn link_at(&self, x: i32, y: i32, scroll: i32) -> Option<&str> {
        let f = self.frags.iter().rev().find(|f| f.href.is_some() && hit(f, x, y, scroll))?;
        f.href.as_deref()
    }

    /* Topmost DOM node under the point, for event dispatch. */
    pub fn hit_node(&self, x: i32, y: i32, scroll: i32) -> Option<usize> {
        let f = self.frags.iter().rev().find(|f| f.node != 0 && hit(f, x, y, scroll))?;
        Some(f.node)
    }
}

fn hit(f: &Fragment, x: i32, y: i32, scroll: i32) -> bool {
    let sy = frag_screen_y(f.y, f.fixed, f.sticky, scroll);
    x >= f.x && x < f.x.saturating_add(f.w) && y >= sy && y < sy.saturating_add(f.h)
}
