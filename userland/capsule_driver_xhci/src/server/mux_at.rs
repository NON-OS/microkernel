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

//! The controller behind one of the multiplexer's slot ids, with one
//! controller or several.

use super::mux::{Mux, VSlot};

impl Mux {
    /// The controller and its own slot behind slot id `s`.
    pub(super) fn at(&self, s: u8) -> Option<(usize, u8)> {
        if self.ctxs.len() == 1 {
            return Some((0, s));
        }
        match self.vslots[s as usize] {
            VSlot::Bound { ctrl, slot } => Some((ctrl as usize, slot)),
            _ => None,
        }
    }

    pub(super) fn addressed(&self, s: u8) -> Option<(usize, u8)> {
        let (c, l) = self.at(s)?;
        let d = &self.ctxs[c].driver;
        d.slots.is_addressed(l, d.layout.max_slots).then_some((c, l))
    }

    /// Whether slot id `s` is enabled and not yet addressed.
    pub(super) fn fresh(&self, s: u8) -> bool {
        if self.ctxs.len() > 1 {
            return matches!(self.vslots[s as usize], VSlot::Pending { .. });
        }
        let d = &self.ctxs[0].driver;
        d.slots.is_allocated(s, d.layout.max_slots) && !d.slots.is_addressed(s, d.layout.max_slots)
    }
}
