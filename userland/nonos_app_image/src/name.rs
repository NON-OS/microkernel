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

//! A member's name: an Alpine package name as the device's index reads it,
//! from a letter or digit, of letters, digits and `._+-` (Alpine has upper
//! case names, `libSvtAv1Enc`), so it is a store name with no path in it.

use crate::format::NAME_MAX;

pub fn valid(name: &[u8]) -> bool {
    let fits = |c: &u8| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'_' | b'+' | b'-');
    match name.first() {
        Some(c) if c.is_ascii_alphanumeric() => {
            name.len() <= NAME_MAX && name.iter().all(fits)
        }
        _ => false,
    }
}
