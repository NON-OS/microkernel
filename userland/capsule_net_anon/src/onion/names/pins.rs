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


//! Resolving a short name, and holding it to the service it first named.

extern crate alloc;

use alloc::vec::Vec;

use super::document::{is_short_name, NameList};

/// Names pinned at once. A boot that resolves more short names than this
/// is refused new ones rather than forgetting an old pin, since a
/// forgotten pin is how a name could quietly move.
pub const PINS_MAX: usize = 1024;

/// Why a name did not resolve.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Resolve {
    /// Not a short name at all.
    NotAName,
    /// No current list yet; one is being fetched.
    Pending,
    /// The current list does not have it.
    Unknown,
    /// It names a different service than it did earlier this boot.
    Changed,
    /// The pin table is full.
    Full,
}

#[derive(Default)]
pub struct Pins {
    pins: Vec<(Vec<u8>, [u8; 32])>,
}

impl Pins {
    /// The service `name` (already in normal form) names, from the built-in
    /// names first and then `list` if it is current at `now`, and the pin
    /// that keeps it there.
    pub fn resolve(&mut self, name: &[u8], builtin: &NameList, list: Option<&NameList>, now: u64) -> Result<[u8; 32], Resolve> {
        if !is_short_name(name) {
            return Err(Resolve::NotAName);
        }
        let found = match builtin.lookup(name) {
            Some(id) => id,
            None => {
                let list = list.filter(|l| now <= l.valid_until).ok_or(Resolve::Pending)?;
                list.lookup(name).ok_or(Resolve::Unknown)?
            }
        };
        match self.pins.iter().find(|(n, _)| n.as_slice() == name) {
            Some((_, pinned)) if *pinned != found => Err(Resolve::Changed),
            Some(_) => Ok(found),
            None if self.pins.len() >= PINS_MAX => Err(Resolve::Full),
            None => {
                self.pins.push((name.to_vec(), found));
                Ok(found)
            }
        }
    }
}

/// Whether `incoming` may replace `current`: never with an older one. The
/// same publication time is allowed, so a list fetched again is taken.
pub fn may_replace(current: Option<&NameList>, incoming: &NameList) -> bool {
    current.is_none_or(|c| incoming.published >= c.published)
}
