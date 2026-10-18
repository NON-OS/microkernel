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

//! A name as a list holds it, one spelling for one name.

/// Lowercase labels of letters, digits and hyphens, 1 to 63 bytes, none
/// starting or ending with a hyphen, joined by single dots, 253 bytes at
/// most, no trailing dot. A last label of digits alone reads as an address,
/// which a list writes as one.
pub fn canonical(name: &[u8]) -> bool {
    let last = name.rsplit(|b| *b == b'.').next().unwrap_or_default();
    !name.is_empty()
        && name.len() <= 253
        && name.split(|b| *b == b'.').all(label)
        && !last.iter().all(u8::is_ascii_digit)
}

fn label(l: &[u8]) -> bool {
    (1..=63).contains(&l.len())
        && l.first() != Some(&b'-')
        && l.last() != Some(&b'-')
        && l.iter().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
}
