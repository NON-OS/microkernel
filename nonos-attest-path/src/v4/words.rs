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

use crate::trailer::bytes_to_digest;

/// A root's four field words, as the STARK statement takes them: `None` unless
/// each little-endian word is canonical, the same rule the path reader applies.
pub fn root_words(root: &[u8; 32]) -> Option<[u64; 4]> {
    bytes_to_digest(root).map(|d| d.map(|w| w.value()))
}
