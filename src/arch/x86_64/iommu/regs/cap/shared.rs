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

//! What a set of remapping units supports together. The kernel builds one set
//! of tables and points every unit at it, so a table feature is usable only
//! when each unit reports it: a depth one unit cannot walk, or a leaf bit one
//! unit treats as reserved, faults every transfer behind that unit.

use super::agaw::{preferred_levels, AgawLevels};
use super::limits::{domain_count, max_address_width};

/// The capability or extended capability bits every word in `words` sets.
/// Meaningful for single-bit features (SAGAW, SLLPS, ECAP.C, ECAP.SC), not
/// for multi-bit fields, which have their own minimum below.
pub fn all_support(words: &[u64]) -> u64 {
    words.iter().fold(!0u64, |acc, word| acc & word)
}

/// The paging depth every unit can walk, preferring four levels as a single
/// unit does. `None` for an empty set or units with no depth in common.
pub fn shared_levels(caps: &[u64]) -> Option<AgawLevels> {
    if caps.is_empty() {
        return None;
    }
    preferred_levels(all_support(caps))
}

/// The narrowest guest address width among the units.
pub fn shared_address_width(caps: &[u64]) -> u8 {
    caps.iter().map(|cap| max_address_width(*cap)).min().unwrap_or(0)
}

/// The fewest domain ids any unit supports; ids are shared across units.
pub fn shared_domain_count(caps: &[u64]) -> u32 {
    caps.iter().map(|cap| domain_count(*cap)).min().unwrap_or(0)
}
