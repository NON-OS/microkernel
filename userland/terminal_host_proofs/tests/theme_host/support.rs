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

//! Luma, contrast gap and the list of shipped profiles.

use crate::term::theme::profiles::{ABYSS, DARK, DIM, LIGHT};
use crate::term::theme::types::Theme;

pub fn luma(c: u32) -> u32 {
    let (r, g, b) = ((c >> 16) & 0xFF, (c >> 8) & 0xFF, c & 0xFF);
    (r * 30 + g * 59 + b * 11) / 100
}

pub fn gap(a: u32, b: u32) -> u32 {
    luma(a).abs_diff(luma(b))
}

pub fn profiles() -> [(&'static str, Theme); 4] {
    [("DARK", DARK), ("DIM", DIM), ("LIGHT", LIGHT), ("ABYSS", ABYSS)]
}
