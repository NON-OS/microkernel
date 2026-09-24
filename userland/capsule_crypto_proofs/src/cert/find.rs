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

//! Locating a byte string, which a certificate is navigated by.

extern crate alloc;

pub(super) fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack
        .len()
        .checked_sub(needle.len())
        .and_then(|last| (from..=last).find(|at| &haystack[*at..*at + needle.len()] == needle))
}
