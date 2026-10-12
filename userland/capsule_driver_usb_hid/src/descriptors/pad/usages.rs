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
//! The usages given before a main item: listed one by one, or as a range,
//! each with its page.

use alloc::vec::Vec;

use super::items::Item;

#[derive(Default)]
pub struct Usages {
    pub list: Vec<(u16, u16)>,
    pub min: Option<(u16, u16)>,
    pub max: Option<(u16, u16)>,
}

impl Usages {
    /// The usage of field `i`: listed, else from the range, else the last.
    pub fn of(&self, i: u32) -> Option<(u16, u16)> {
        if let Some(&u) = self.list.get(i as usize) {
            return Some(u);
        }
        if let (Some((page, lo)), Some((_, hi))) = (self.min, self.max) {
            let at = lo as u32 + i;
            return (at <= hi as u32).then_some((page, at as u16));
        }
        self.list.last().copied()
    }
}

/// A usage, with its page: a four-byte usage carries its own.
pub fn usage(it: &Item, page: u16) -> (u16, u16) {
    if it.size == 4 {
        ((it.value >> 16) as u16, it.value as u16)
    } else {
        (page, it.value as u16)
    }
}
