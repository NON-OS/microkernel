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

//! Sizes as a drive's label gives them: decimal units, one decimal place.

use alloc::format;
use alloc::string::String;

const UNITS: [&str; 5] = ["KB", "MB", "GB", "TB", "PB"];

pub fn size_text(bytes: u64) -> String {
    if bytes < 1000 {
        return format!("{bytes} B");
    }
    let (mut unit, mut scale) = (0usize, 1000u64);
    while unit + 1 < UNITS.len() && bytes / scale >= 1000 {
        scale *= 1000;
        unit += 1;
    }
    let tenths = (bytes as u128 * 10 + scale as u128 / 2) / scale as u128;
    format!("{}.{} {}", tenths / 10, tenths % 10, UNITS[unit])
}
