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

//! A string from a structure, made safe to show: SMBIOS strings are bytes
//! with no promised encoding, so anything outside printable ASCII becomes
//! '?', surrounding blanks go, and length is capped.

use alloc::string::String;

pub const MAX_TEXT: usize = 64;

pub fn text(s: &[u8]) -> String {
    let shown: String = s
        .iter()
        .take(MAX_TEXT)
        .map(|b| if (0x20..0x7F).contains(b) { *b as char } else { '?' })
        .collect();
    String::from(shown.trim())
}
