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

/* FNV-1a over a name, ASCII-lowercased first when `lower` (tag names,
 * attribute names and custom property names compare that way; ids and
 * classes compare exactly). Names are then compared as integers, which
 * costs one multiply per byte instead of a string compare per probe. */
pub(in crate::browser::css) fn hash_name(name: &str, lower: bool) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &b in name.as_bytes() {
        let b = if lower { b.to_ascii_lowercase() } else { b };
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}
