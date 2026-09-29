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

use crate::browser::css::parse_color;

use super::brush::fade;

/* currentColor maps to a light neutral: the embedding text colour is not
 * known at decode time and icons land on the dark theme far more often
 * than not. */
pub(super) const CURRENT_COLOR: u32 = 0xFFC8_D2DC;

/// A paint that names a colour, as ARGB: every CSS colour syntax, with
/// none and transparent painting nothing (Some(None)); None when the
/// value is not a colour at all.
pub(super) fn parse_paint(v: &str, current: u32) -> Option<Option<u32>> {
    let t = v.trim();
    if t.eq_ignore_ascii_case("none") {
        return Some(None);
    }
    if t.eq_ignore_ascii_case("currentcolor") {
        return Some((current >> 24 != 0).then_some(current));
    }
    parse_color(t).map(|c| (c >> 24 != 0).then_some(c))
}

/// A gradient stop's colour with its stop-opacity folded into alpha.
pub(super) fn stop_color(v: &str, opacity: f32) -> u32 {
    match parse_paint(v, CURRENT_COLOR) {
        Some(Some(c)) => fade(c, opacity),
        Some(None) => 0,
        /* An unreadable stop-color takes its initial value, black. */
        None => fade(0xff00_0000, opacity),
    }
}
