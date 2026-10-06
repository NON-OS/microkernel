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

use crate::browser::css::decl::Decl;

use super::Rule;

impl Rule {
    /* What one page's rules may keep across its sheets: complex selectors,
     * and bytes as Rule::cost estimates them, which nesting, :is() and
     * long lists multiply a sheet's text into. A rule that would pass
     * either fills the sheet: it and every later rule are dropped. */
    pub const MAX_SELECTORS: usize = 65_536;
    pub const MAX_BYTES: usize = 16 << 20;
}

/* The Rule flags one declaration implies. */
pub(super) fn decl_flags(d: &Decl) -> u16 {
    let mut f = if d.important { Rule::IMPORTANT } else { 0 };
    if d.flags & Decl::CUSTOM != 0 {
        f |= Rule::CUSTOM;
    }
    match d.name.as_str() {
        "font-size" => f |= Rule::FONT_SIZE,
        "content" => f |= Rule::CONTENT,
        _ => {}
    }
    if d.name.starts_with("counter-") || d.value.contains("counter") {
        f |= Rule::COUNTERS;
    }
    /* A number followed by v and w, h, m(in/ax), i or b: 100vw, 50dvh. */
    let unit = |w: &[u8]| {
        (w[0].is_ascii_digit() || w[0] == b'.' || w[0] == b'd' || w[0] == b's' || w[0] == b'l')
            && w[1] == b'v'
            && matches!(w[2], b'w' | b'h' | b'm' | b'i' | b'b')
    };
    if d.value.as_bytes().windows(3).any(unit) {
        f |= Rule::VP_UNITS;
    }
    f
}
