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

use super::super::inline_items::InlineItem;
use super::inline_parts::Lead;

impl InlineItem {
    /* (advance, height, lead) of any item; a break has none. */
    fn dims(&self) -> (i32, i32, Lead) {
        match self {
            InlineItem::Word { adv, line_h, lead, .. } => (*adv, *line_h, *lead),
            InlineItem::Image { w, h, lead, .. } | InlineItem::Atom { w, h, lead, .. } => {
                (*w, *h, *lead)
            }
            InlineItem::Break => (0, 0, Lead::default()),
        }
    }

    pub(in super::super) fn advance_w(&self) -> i32 {
        self.dims().0
    }

    pub(in super::super) fn item_h(&self) -> i32 {
        self.dims().1
    }

    pub(in super::super) fn lead(&self) -> Lead {
        self.dims().2
    }

    /* The width of the space before this item, 0 when the source has none. */
    pub(in super::super) fn space_w(&self) -> i32 {
        self.dims().2.space
    }
}
