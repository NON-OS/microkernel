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

/* <font size>: 1 to 7, or relative to 3 with a sign. */
pub(super) fn font_size(v: &str) -> Option<&'static str> {
    const SIZES: [&str; 7] =
        ["x-small", "small", "medium", "large", "x-large", "xx-large", "xxx-large"];
    let v = v.trim();
    let n: i32 = v.trim_start_matches(['+', '-']).parse().ok()?;
    let n = if v.starts_with('+') {
        3 + n
    } else if v.starts_with('-') {
        3 - n
    } else {
        n
    };
    Some(SIZES[(n.clamp(1, 7) - 1) as usize])
}
