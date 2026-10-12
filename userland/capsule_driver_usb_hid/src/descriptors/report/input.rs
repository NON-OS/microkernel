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

//! One Input item's fields as the collection kinds read them: its report,
//! the application collection it is in, its first bit, and the globals and
//! usages in force.

use super::globals::Globals;
use super::layout::{Field, Slots};
use super::locals::Locals;

pub struct Input<'a> {
    pub id: u8,
    /// Which application collection, counted from 1 in descriptor order.
    pub app: u16,
    pub at: u32,
    pub g: Globals,
    pub variable: bool,
    pub relative: bool,
    pub l: &'a Locals,
}

impl Input<'_> {
    /// Where field `i` starts. The report's bits are bounded by
    /// MAX_REPORT_BITS, so it fits.
    pub fn field_at(&self, i: u32) -> u16 {
        (self.at + i * self.g.size) as u16
    }

    pub fn field(&self, i: u32) -> Field {
        let g = &self.g;
        Field {
            at: self.field_at(i),
            size: g.size as u8,
            min: g.min,
            max: g.max,
            relative: self.relative,
        }
    }

    /// The item as array slots, when it has 1 to 6 slots of 1 to 16 bits.
    pub fn slots(&self) -> Option<Slots> {
        let g = &self.g;
        let fits = (1..=16).contains(&g.size) && (1..=6).contains(&g.count);
        let slots = Slots {
            at: self.at as u16,
            size: g.size as u8,
            count: g.count as u8,
            min: g.min,
            max: g.max,
        };
        fits.then_some(slots)
    }
}
