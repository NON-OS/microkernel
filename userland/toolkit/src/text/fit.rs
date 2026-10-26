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
//! The frame's measures at one to one (HANDOFF/ui/identity.md, Window
//! A line cut to a width with an ellipsis, never through a character.

use alloc::string::String;

pub const ELLIPSIS: &str = "\u{2026}";

/// `s` whole when `measure` says it fits in `max_w`; else its longest
/// leading run, cut on a character boundary, with the ellipsis after it
/// that fits (the ellipsis alone, last); else nothing.
pub fn fit_end(s: &str, max_w: u32, measure: impl Fn(&str) -> u32) -> String {
    if measure(s) <= max_w {
        return String::from(s);
    }
    let mut cut = s.len();
    while cut > 0 {
        cut = s[..cut].char_indices().next_back().map_or(0, |(i, _)| i);
        let mut t = String::from(s[..cut].trim_end());
        t.push_str(ELLIPSIS);
        if measure(&t) <= max_w {
            return t;
        }
    }
    String::new()
}
