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

use super::viewport;
use crate::browser::fonts::{ch_px, ex_px};

/* CSS absolute units are defined against the 96 px inch. */
const PX_PER_IN: f32 = 96.0;

/// Pixels in one `unit` (lowercase ASCII) for an element whose font-size is
/// `em` px, or None when the unit is not a length. Viewport units use the
/// viewport the running cascade published; a desktop window has no dynamic
/// toolbars, so the small, large and dynamic variants all equal the plain one.
pub(super) fn unit_px(unit: &[u8], em: f32) -> Option<f32> {
    let vw = viewport::width() as f32 / 100.0;
    let vh = viewport::height() as f32 / 100.0;
    Some(match unit {
        b"px" => 1.0,
        b"em" => em,
        b"rem" => viewport::root_font(),
        b"ch" => ch_px(em),
        b"ex" => ex_px(em),
        b"vw" | b"svw" | b"lvw" | b"dvw" => vw,
        b"vh" | b"svh" | b"lvh" | b"dvh" => vh,
        b"vmin" | b"svmin" | b"lvmin" | b"dvmin" => vw.min(vh),
        b"vmax" | b"svmax" | b"lvmax" | b"dvmax" => vw.max(vh),
        b"in" => PX_PER_IN,
        b"cm" => PX_PER_IN / 2.54,
        b"mm" => PX_PER_IN / 25.4,
        b"q" => PX_PER_IN / 101.6,
        b"pt" => PX_PER_IN / 72.0,
        b"pc" => PX_PER_IN / 6.0,
        _ => return None,
    })
}
