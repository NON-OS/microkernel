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

//! The global items as they come: each holds until changed, and Push and
//! Pop save and restore them all, on a stack of bounded depth.

use super::globals::Globals;
use super::item::Item;
use super::limits::{Refused, MAX_FIELD_BITS, MAX_PUSH, MAX_REPORT_BITS};

#[derive(Default)]
pub struct GlobalState {
    pub now: Globals,
    stack: [Globals; MAX_PUSH],
    pushed: usize,
    /// A Report ID item was seen: every report starts with its ID byte.
    pub ids: bool,
}

impl GlobalState {
    /// Take one global item. Err for a Report ID of 0, which is reserved, a
    /// size or count past its bound, or a Push or Pop the stack cannot take.
    pub fn take(&mut self, item: Item) -> Result<(), Refused> {
        match item.tag {
            0 => self.now.page = item.data as u16,
            1 => self.now.min = item.signed(),
            // A maximum over a minimum of zero or more is unsigned, as a
            // one-byte 0xFF for 255 is commonly written (Linux reads it so).
            2 if self.now.min < 0 => self.now.max = item.signed(),
            2 => self.now.max = i64::from(item.data),
            7 if item.data <= MAX_FIELD_BITS => self.now.size = item.data,
            8 if (1..=255).contains(&item.data) => {
                self.now.id = item.data as u8;
                self.ids = true;
            }
            9 if item.data <= MAX_REPORT_BITS => self.now.count = item.data,
            7..=9 => return Err(Refused),
            10 => {
                *self.stack.get_mut(self.pushed).ok_or(Refused)? = self.now;
                self.pushed += 1;
            }
            11 => {
                self.pushed = self.pushed.checked_sub(1).ok_or(Refused)?;
                self.now = self.stack[self.pushed];
            }
            _ => {}
        }
        Ok(())
    }
}
