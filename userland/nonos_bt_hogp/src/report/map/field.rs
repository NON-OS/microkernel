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

//! What a main item leaves behind: a run of report bits and the usages it
//! carries (HID 1.11, 6.2.2.4 and 6.2.2.8).

use super::global::Globals;
use super::span::{Dir, Span};
use alloc::vec::Vec;

/// One field: `count` elements of `size` bits from `bit`, counted from
/// the start of the report after its ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub dir: Dir,
    pub id: u8,
    pub bit: u32,
    pub size: u32,
    pub count: u32,
    /// An array field's elements hold usage indexes; a variable field's
    /// elements are each one usage's value.
    pub array: bool,
    pub lmin: i32,
    pub lmax: i64,
    pub spans: Vec<Span>,
}

impl Field {
    /// A field from a main item's flags (bit 1 clear for an array) and
    /// the globals it was laid out with.
    pub(super) fn new(dir: Dir, g: &Globals, bit: u32, flags: u32, spans: Vec<Span>) -> Self {
        let (id, size, count, lmin, lmax) = (g.id, g.size, g.count, g.lmin, g.max());
        Self { dir, id, bit, size, count, array: flags & 2 == 0, lmin, lmax, spans }
    }

    /// The usage at `index` in the field's list, as (page, usage).
    pub fn usage(&self, index: u32) -> Option<(u16, u16)> {
        let mut left = index;
        for s in &self.spans {
            if left < s.usages() {
                return Some((s.page, s.min + left as u16));
            }
            left -= s.usages();
        }
        None
    }

    /// The usage of a variable field's element `i`: the last usage covers
    /// any elements past the list (HID 1.11, 6.2.2.8).
    pub fn usage_of(&self, i: u32) -> Option<(u16, u16)> {
        let total: u32 = self.spans.iter().map(Span::usages).sum();
        self.usage(i.min(total.checked_sub(1)?))
    }
}
