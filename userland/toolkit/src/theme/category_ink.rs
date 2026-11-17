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

//! The ink for text drawn on a category's fill.

use super::category::Category;
use super::colour::Opaque;
use super::contrast::{at_least, BODY};
use super::roles::{roles, Roles};

impl Category {
    /// Text drawn on its fill in the theme in force.
    pub fn ink(self) -> Opaque {
        self.ink_in(&roles())
    }

    /// Text drawn on its fill under the roles `r`.
    pub fn ink_in(self, r: &Roles) -> Opaque {
        self.ink_over(&flat(r), r.text.argb())
    }

    /// The first of `grounds`, `text`, black and white that reads on the fill at
    /// 4.5:1. One of black or white always does, so the search cannot run dry.
    pub fn ink_over(self, grounds: &[u32], text: u32) -> Opaque {
        let fill = self.colour_over(grounds, text).argb();
        let mut inks = grounds.iter().copied().chain([text, 0xFF00_0000, 0xFFFF_FFFF]);
        Opaque::new(inks.find(|&i| at_least(i, fill, BODY)).unwrap_or(0xFFFF_FFFF))
    }
}

/// The four flat grounds of `r`, which a category may be drawn on.
pub(super) fn flat(r: &Roles) -> [u32; 4] {
    [r.bg, r.sunken, r.surface, r.raised].map(Opaque::argb)
}
