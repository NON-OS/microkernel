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

//! Painting the hint bar: its band, its hairline, and the pairs that fit.

use nonos_app_skeleton::PaintBuffer;

use super::{footer_h, HINT_PX, KEY_GAP, PAD_TOP, PAIR_GAP};
use crate::paint::hint_sets::{hints, Mode};
use crate::paint::constants::TEXT_LEFT;
use crate::paint::shade::elevate;
use crate::term::theme::types::Theme;

pub fn draw_footer(fb: &mut PaintBuffer, mode: Mode, t: &Theme) {
    let band = footer_h();
    let y = fb.height.saturating_sub(band);
    fb.fill_rect(0, y, fb.width, band, elevate(t.bg, 10));
    // A hairline rather than a block in another shade. It separates the bar
    // from the body without spending a row of height to do it.
    fb.fill_rect(0, y, fb.width, 1, elevate(t.bg, 22));

    let baseline = (y + PAD_TOP) as i32;
    let mut x = TEXT_LEFT as i32;
    for (key, label) in hints(mode) {
        let key_w = fb.measure_ttf(key, HINT_PX);
        let label_w = fb.measure_ttf(label, HINT_PX);
        // Stop before a pair would run under the right edge, rather than
        // clipping one in half.
        if x + key_w + KEY_GAP + label_w > fb.width as i32 - TEXT_LEFT as i32 {
            break;
        }
        let _ = fb.text_ttf(x, baseline, key, t.accent, HINT_PX);
        x += key_w + KEY_GAP;
        let _ = fb.text_ttf(x, baseline, label, t.dim, HINT_PX);
        x += label_w + PAIR_GAP;
    }
}
