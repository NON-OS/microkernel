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

use crate::browser::css::computed::Computed;

/* aspect-ratio: `auto`, a ratio `w / h`, or a single number `w` (over 1).
 * `auto w / h` uses the ratio, since a non-replaced box has no natural one.
 * Layout derives an auto height from the width with it. */
pub(super) fn apply_aspect(c: &mut Computed, name: &str, value: &str) -> bool {
    if name != "aspect-ratio" {
        return false;
    }
    let v = value.trim();
    let ratio = v.strip_prefix("auto").map_or(v, str::trim);
    let ratio = ratio.strip_suffix("auto").map_or(ratio, str::trim);
    if ratio.is_empty() {
        c.aspect = None;
        return true;
    }
    let (w, h) = match ratio.split_once('/') {
        Some((w, h)) => (w.trim(), h.trim()),
        None => (ratio, "1"),
    };
    let num = |s: &str| s.parse::<f32>().ok().filter(|f| f.is_finite() && *f > 0.0);
    if let (Some(w), Some(h)) = (num(w), num(h)) {
        c.aspect = Some(w / h);
    }
    true
}
