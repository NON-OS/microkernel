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

use super::color::args::{args, Args};
use super::color::rgbaf::Rgbaf;

/* rgb()/rgba() arguments, legacy `r, g, b[, a]` or modern `r g b [/ a]`:
 * each channel a number 0..255 or a percentage, alpha a number 0..1 or a
 * percentage. Returns ARGB with the alpha kept. */
pub(super) fn parse_rgb(inner: &str) -> Option<u32> {
    let Args { c, alpha } = args(inner)?;
    let rgb = c.map(|v| v.scaled(255.0) / 255.0);
    Some(Rgbaf { rgb, a: alpha }.to_argb())
}
