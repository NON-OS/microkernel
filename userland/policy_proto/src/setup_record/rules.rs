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

/*
 * What a kept name and a kept tier may hold. The name rules are the ones
 * setup's name step types under; the tier is a pinned tier's name. Empty is
 * allowed for both: no name given, no tier chosen.
 */

pub const NAME_MAX: usize = 32;
pub const TIER_MAX: usize = 24;

/* Lowercase letters, digits, - and _, a letter first, at most `NAME_MAX`. */
pub fn name_ok(name: &[u8]) -> bool {
    match name.split_first() {
        None => true,
        Some((first, rest)) => {
            name.len() <= NAME_MAX
                && first.is_ascii_lowercase()
                && rest.iter().all(|&c| {
                    c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-' || c == b'_'
                })
        }
    }
}

/* Lowercase letters, digits, - and ., at most `TIER_MAX`. */
pub fn tier_ok(tier: &[u8]) -> bool {
    tier.len() <= TIER_MAX
        && tier
            .iter()
            .all(|&c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-' || c == b'.')
}
