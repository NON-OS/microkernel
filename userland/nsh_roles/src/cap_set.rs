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

//! A set of kernel capabilities, as a mask over their bits. A set is only
//! made from a parsed name list, so each member is a name the kernel has;
//! a ceiling is a set that also holds none of the forbidden nine.

use crate::caps::{bit, forbidden_mask, KERNEL_CAPS};
use alloc::vec::Vec;
use nsh_wire::CapNames;

pub use crate::error::CapError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CapSet(u64);

impl CapSet {
    /// The set a list names. An unknown name is refused; whether the set
    /// is allowed is a question for a ceiling.
    pub fn named(names: &CapNames<'_>) -> Result<CapSet, CapError> {
        names.iter().try_fold(CapSet(0), |set, name| match bit(name) {
            Some(b) => Ok(CapSet(set.0 | 1 << b)),
            None => Err(CapError::Unknown),
        })
    }

    /// A role's ceiling: known names, and none of the forbidden ones.
    pub fn ceiling(names: &CapNames<'_>) -> Result<CapSet, CapError> {
        let set = CapSet::named(names)?;
        match set.0 & forbidden_mask() {
            0 => Ok(set),
            _ => Err(CapError::Forbidden),
        }
    }

    /// Every member of `self` is in `ceiling`.
    pub const fn within(self, ceiling: CapSet) -> bool {
        self.0 & !ceiling.0 == 0
    }

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, name: &[u8]) -> bool {
        bit(name).is_some_and(|b| self.0 >> b & 1 == 1)
    }

    /// The names in the set, in ascending byte order, as a grant lists them.
    pub fn names(self) -> Vec<&'static str> {
        let set = KERNEL_CAPS.iter().enumerate().filter(|(i, _)| self.0 >> i & 1 == 1);
        let mut out: Vec<&'static str> = set.map(|(_, n)| *n).collect();
        out.sort_unstable();
        out
    }
}
