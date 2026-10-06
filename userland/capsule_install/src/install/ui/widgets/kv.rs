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

use alloc::string::String;

use crate::install::ui::metrics::Metrics;
use crate::install::ui::text::{fit, top_of};
use crate::install::ui::{text, theme};
use nonos_brand::label as tag;

/// Paints one row at `y` and returns the y of the next.
#[allow(clippy::too_many_arguments)]
pub fn kv(
    fb: &mut PaintBuffer,
    m: &Metrics,
    x: u32,
    y: u32,
    w: u32,
    label: &str,
    value: &str,
    mono: bool,
) -> u32 {
    let line = m.line_h;
    let caps: String = label.chars().map(|c| c.to_ascii_uppercase()).collect();
    tag(fb, x, top_of(y, line, m.label_px), &caps, theme::MUTED, m.label_px);
    let vx = x + m.label_w;
    let vw = w.saturating_sub(m.label_w);
    if mono {
        let cut = fit(fb, value, m.mono_px, vw);
        text::mono(fb, vx, top_of(y, line, m.mono_px), cut, theme::FOREGROUND, m.mono_px);
    } else {
        let cut = fit(fb, value, m.body_px, vw);
        text::line(fb, vx, top_of(y, line, m.body_px), cut, theme::FOREGROUND, m.body_px);
    }
    y + line
}
