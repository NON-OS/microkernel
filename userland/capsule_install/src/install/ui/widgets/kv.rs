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

//! A label and its value on one line. Hashes and identifiers go in the
//! mono face so a person can compare them against a boot log by eye.

use nonos_app_skeleton::PaintBuffer;

use crate::install::ui::metrics::{BODY_PX, LINE_H, MONO_PX};
use crate::install::ui::text::{fit, top_of};
use crate::install::ui::{text, theme};

const LABEL_W: u32 = 170;

/// Paints one row at `y` and returns the y of the next.
pub fn kv(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    label: &str,
    value: &str,
    mono: bool,
) -> u32 {
    let top = top_of(y, LINE_H, BODY_PX);
    text::line(fb, x, top, label, theme::MUTED, BODY_PX);
    let vx = x + LABEL_W;
    let vw = w.saturating_sub(LABEL_W);
    if mono {
        let cut = fit(fb, value, MONO_PX, vw);
        text::mono(fb, vx, top_of(y, LINE_H, MONO_PX), cut, theme::FOREGROUND, MONO_PX);
    } else {
        let cut = fit(fb, value, BODY_PX, vw);
        text::line(fb, vx, top, cut, theme::FOREGROUND, BODY_PX);
    }
    y + LINE_H
}
