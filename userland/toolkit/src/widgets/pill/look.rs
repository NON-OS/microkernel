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

//! The colours a pill draws its body, dot, label and count in.

use super::Pill;
use crate::theme::derive::away;
use crate::theme::{Category, Opaque, Roles};
use crate::widgets::{draw, Tone};

/// How far a hovered filled body moves from its ink, out of 255: half a press.
const HOVER: u8 = 16;

impl Pill<'_> {
    /// The category of a live filled pill; None otherwise.
    pub(super) fn fill_by(&self) -> Option<Category> {
        self.category.filter(|_| self.filled && !self.state.disabled)
    }

    /// The body: the category's colour on a live filled pill, else `raised`;
    /// moved away from the label's ink while pressed, and a filled body half
    /// as far while hovered, in place of a wash that could cost it contrast.
    pub(super) fn body_fill(&self, r: &Roles) -> Opaque {
        let by = self.fill_by();
        let (fill, ink) = by.map_or((r.raised, r.text), |c| (c.colour_in(r), c.ink_in(r)));
        match (self.state.disabled, self.state.pressed, self.state.hovered && by.is_some()) {
            (false, true, _) => draw::pressed(fill, ink),
            (false, false, true) => Opaque::new(away(fill.argb(), ink.argb(), HOVER)),
            _ => fill,
        }
    }

    /// The dot: the category's colour, else the tone's; `disabled` when
    /// disabled.
    pub(super) fn dot_ink(&self, r: &Roles) -> Opaque {
        match (self.state.disabled, self.category) {
            (true, _) => r.disabled,
            (false, Some(c)) => c.colour_in(r),
            (false, None) => self.tone.colour(r),
        }
    }

    /// The label: the fill's ink on a filled pill, `text` beside a dot or
    /// for `Plain`, else the tone's colour; `disabled` when disabled.
    pub(super) fn label_ink(&self, r: &Roles) -> Opaque {
        match self.fill_by() {
            Some(c) => c.ink_in(r),
            None if self.state.disabled => r.disabled,
            None if self.dot || self.tone == Tone::Plain => r.text,
            None => self.tone.colour(r),
        }
    }

    /// The count: the fill's ink on a filled pill, else `muted`, or `disabled`.
    pub(super) fn count_ink(&self, r: &Roles) -> Opaque {
        match self.fill_by() {
            Some(c) => c.ink_in(r),
            None if self.state.disabled => r.disabled,
            None => r.muted,
        }
    }
}
