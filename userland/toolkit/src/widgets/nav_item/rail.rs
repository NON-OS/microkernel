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

//! A compact navigation item's layout: its icon alone, centred.

use super::geometry::Parts;
use super::NavItem;
use crate::decorations::Rect;
use crate::design::scale::scale;
use crate::design::size::{CONTROL_RADIUS, ICON};
use crate::widgets::rect::centre;
use crate::widgets::text;

impl NavItem<'_> {
    /// The body is `at` as given, with any icon an `ICON` square centred in
    /// it, an empty label box and no count.
    pub(super) fn rail(&self, at: Rect) -> Parts {
        let s = scale();
        let side = s.px(ICON).min(at.h).min(at.w);
        let icon = self.icon.map(|_| centre(at, side, side));
        let label = Rect { w: 0, ..centre(at, 0, at.h) };
        let px = Self::px();
        let label_top = text::centre_top(at.y, at.h, px);
        Parts { body: at, radius: s.px(CONTROL_RADIUS), icon, label, count: None, label_top, px }
    }
}
