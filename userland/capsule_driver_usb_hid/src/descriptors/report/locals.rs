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

//! The local items (HID 1.11, 6.2.2.8): the usages of the next main item,
//! listed or as a range. A usage of one or two bytes is on the usage page
//! of the moment; one of four bytes names its page in its top half.

use super::item::Item;
use super::limits::MAX_USAGES;

#[derive(Default)]
pub struct Locals {
    list: [u32; MAX_USAGES],
    n: usize,
    min: Option<u32>,
    max: Option<u32>,
}

impl Locals {
    pub fn take(&mut self, item: Item, page: u16) {
        let usage = if item.size == 4 { item.data } else { u32::from(page) << 16 | item.data };
        match item.tag {
            0 if self.n < MAX_USAGES => {
                self.list[self.n] = usage;
                self.n += 1;
            }
            1 => self.min = Some(usage),
            2 => self.max = Some(usage),
            _ => {}
        }
    }

    /// The usage of field `i`: from the list, the last one again past its
    /// end; else from the range, its top again past it; else none (0).
    pub fn usage(&self, i: u32) -> u32 {
        if self.n > 0 {
            return self.list[(i as usize).min(self.n - 1)];
        }
        match self.range() {
            Some((lo, hi)) => lo.saturating_add(i).min(hi),
            None => 0,
        }
    }

    /// The range, when the usages are given as one and not listed.
    pub fn range(&self) -> Option<(u32, u32)> {
        match (self.n, self.min, self.max) {
            (0, Some(lo), Some(hi)) if lo <= hi => Some((lo, hi)),
            _ => None,
        }
    }

    /// Where `usage` stands among the usages: the array value, less the
    /// logical minimum, that names it.
    pub fn index_of(&self, usage: u32) -> Option<u32> {
        if self.n > 0 {
            return self.list[..self.n].iter().position(|&u| u == usage).map(|i| i as u32);
        }
        let (lo, hi) = self.range()?;
        (lo..=hi).contains(&usage).then(|| usage - lo)
    }
}
