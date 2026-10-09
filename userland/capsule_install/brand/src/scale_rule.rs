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

//! The rule from a canvas to its scale, alone, so the desktop shell can mount
//! it by path and draw its type at setup's size without the rest of the brand.

/// Drawing pixels per logical pixel on a canvas `width` by `height`, in
/// quarters, from its short side: 4 is one to one, 5 from 1000, 6 from 1440
/// and 8 from 2160.
pub fn quarters_for(width: u32, height: u32) -> u32 {
    match width.min(height) {
        s if s >= 2160 => 8,
        s if s >= 1440 => 6,
        s if s >= 1000 => 5,
        _ => 4,
    }
}
