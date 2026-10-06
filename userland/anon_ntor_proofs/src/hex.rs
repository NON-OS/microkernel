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

//! Turning the hex in a published test vector into bytes, for the tests only.

/// Decode a hex string. Panics on malformed input, which is correct here: a
/// malformed literal in a test vector is a mistake in the test, and failing
pub fn hex(text: &str) -> Vec<u8> {
    assert!(text.len().is_multiple_of(2), "hex literal has an odd length");
    (0..text.len() / 2)
        .map(|index| {
            u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex literal is not hex")
        })
        .collect()
}
