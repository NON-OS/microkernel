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

use crate::browser::css::{Align, Computed, Size};

use super::super::tree::{BoxKind, BoxNode};

/* How an item lines up in its columns: justify-self, else the container's
 * justify-items. Normal alignment stretches a box; an image keeps its own
 * width at the start. */
pub(in super::super) fn col_align(it: &BoxNode, s: &Computed) -> Align {
    let image = matches!(it.kind, BoxKind::Image { .. });
    let normal = if image { Align::Start } else { Align::Stretch };
    it.style.justify_self.or(s.justify_items).unwrap_or(normal)
}

/* How an item lines up in its rows: align-self, else the container's
 * align-items. Normal alignment stretches a box but not an image, which
 * keeps its own height at the start. */
pub(in super::super) fn row_align(it: &BoxNode, s: &Computed) -> Align {
    let image = matches!(it.kind, BoxKind::Image { .. });
    let a = match it.style.align_self {
        Some(a) => a,
        None if image && s.align == Align::Stretch => Align::Start,
        None => s.align,
    };
    /* Only an auto height stretches. */
    if a == Align::Stretch && it.style.height != Size::Auto {
        Align::Start
    } else {
        a
    }
}
