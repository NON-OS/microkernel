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

const MAX_ATTR_PX: u32 = 4096;

/* Whole-pixel value of an <img> or <svg> width/height attribute: a plain or
 * decimal number, optionally with a px unit, rounded. "0" is a real zero
 * size (a hidden sprite sheet takes no room). Percentages, other units and
 * malformed values yield None and the box falls back to its default size. */
pub(super) fn attr_px(v: Option<&str>) -> Option<u32> {
    let t = v?.trim();
    let n = t.strip_suffix("px").unwrap_or(t).parse::<f32>().ok()?;
    if !n.is_finite() || n < 0.0 {
        return None;
    }
    Some(((n + 0.5) as u32).min(MAX_ATTR_PX))
}
