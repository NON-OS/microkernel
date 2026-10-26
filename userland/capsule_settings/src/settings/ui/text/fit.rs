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

//! A line of text cut to a width, ending in an ellipsis when it is cut, so a
//! long label stops short of the value beside it instead of running under it.
//! Pure over the measure, which is the font's on the device and a fixed
//! advance in the proofs.

use alloc::string::String;

pub fn fit(s: &str, max_w: u32, measure: impl Fn(&str) -> u32) -> String {
    if measure(s) <= max_w {
        return String::from(s);
    }
    let mut end = s.len();
    while end > 0 {
        end = s[..end].char_indices().last().map_or(0, |(i, _)| i);
        let cut = alloc::format!("{}…", s[..end].trim_end());
        if measure(&cut) <= max_w {
            return cut;
        }
    }
    String::new()
}
