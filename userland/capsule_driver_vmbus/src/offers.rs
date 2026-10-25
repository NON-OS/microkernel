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

//! The channels the host has offered and not withdrawn, by relid.

use crate::constants::MAX_CHANNELS;
use crate::msg::Offer;

pub struct Offers {
    slots: [Option<Offer>; MAX_CHANNELS],
    /// Offers that found no room.
    pub dropped: u32,
}

impl Offers {
    pub const fn new() -> Self {
        Self { slots: [None; MAX_CHANNELS], dropped: 0 }
    }

    /// Kept, replacing an offer with the same relid. False when full.
    pub fn insert(&mut self, o: Offer) -> bool {
        let at = self
            .slots
            .iter()
            .position(|s| s.is_some_and(|s| s.relid == o.relid))
            .or_else(|| self.slots.iter().position(Option::is_none));
        match at {
            Some(i) => {
                self.slots[i] = Some(o);
                true
            }
            None => {
                self.dropped = self.dropped.saturating_add(1);
                false
            }
        }
    }

    /// The offer withdrawn, when it was kept.
    pub fn remove(&mut self, relid: u32) -> Option<Offer> {
        let i = self.slots.iter().position(|s| s.is_some_and(|s| s.relid == relid))?;
        self.slots[i].take()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Offer> {
        self.slots.iter_mut().flatten()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Offer> {
        self.slots.iter().flatten()
    }

    pub fn count(&self) -> usize {
        self.iter().count()
    }
}

impl Default for Offers {
    fn default() -> Self {
        Self::new()
    }
}
