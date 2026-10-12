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

//! Report descriptors built item by item, so a test can push one bound.

pub const MOUSE_APP: &[u8] = &[0x05, 0x01, 0x09, 0x02, 0xa1, 0x01];

/// A mouse collection holding `items`, closed.
pub fn app(items: &[&[u8]]) -> Vec<u8> {
    [MOUSE_APP, &items.concat(), &[0xc0]].concat()
}

/// Report Size, Report Count (two data bytes each), then an Input item.
pub fn input(size: u16, count: u16) -> Vec<u8> {
    let ([s0, s1], [c0, c1]) = (size.to_le_bytes(), count.to_le_bytes());
    vec![0x76, s0, s1, 0x96, c0, c1, 0x81, 0x02]
}
