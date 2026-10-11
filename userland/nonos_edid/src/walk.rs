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

//! Every timing the whole EDID names, base block first, then each good
//! extension in order, so the first preferred timing seen is the one
//! the base block (or failing it, DisplayID) prefers.

use crate::cta::blocks::Cta;
use crate::displayid::section::Section;
use crate::edid::Edid;
use crate::timing::Timing;
use crate::{cta, displayid};

impl Edid<'_> {
    pub fn for_each_timing(&self, f: &mut dyn FnMut(Timing)) {
        self.base_timings(f);
        for b in self.extensions() {
            if let Some(c) = Cta::new(b) {
                cta::timings::each(c, f);
            } else if let Some(s) = Section::new(b) {
                displayid::type7::each(s, f);
            }
        }
    }
}
