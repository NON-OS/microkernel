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

//! The local items the mouse walk keeps between main items: the Usages, and
//! a Usage Minimum/Maximum range, which button arrays declare instead.

use super::usage_for::{Usages, MAX_USAGES};

#[derive(Default)]
pub(super) struct Locals {
    usages: Usages,
    min: Option<u16>,
    max: Option<u16>,
}

impl Locals {
    pub fn local(&mut self, tag: u8, data: u32) {
        match tag {
            0x0 if self.usages.n < MAX_USAGES => {
                self.usages.list[self.usages.n] = data as u16;
                self.usages.n += 1;
            }
            0x1 => self.min = Some(data as u16),
            0x2 => self.max = Some(data as u16),
            _ => {}
        }
    }

    /// The usage of field `k` of a main item: the kth Usage, the last one
    /// for the fields past it, or else the kth step of the range.
    pub fn usage(&self, k: u32) -> u16 {
        if self.usages.n > 0 {
            self.usages.list[(k as usize).min(self.usages.n - 1)]
        } else if let Some(min) = self.min {
            let v = min.saturating_add(k as u16);
            self.max.map_or(v, |max| v.min(max))
        } else {
            0
        }
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
