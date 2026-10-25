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

//! The fade laid over a section photograph, row by row.

pub(super) fn fade(row: u32, h: u32) -> u32 {
    let t = row * 1000 / h.max(1);
    let alpha = match t {
        0..=299 => 350 * (300 - t) / 300,
        300..=549 => 0,
        _ => 1000 * (t - 550) / 450,
    };
    alpha.min(1000) * 255 / 1000
}
