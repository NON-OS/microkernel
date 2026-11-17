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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::CARD_RADIUS;
use nonos_toolkit::theme::derive::with_alpha;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::Rect;

/* Logical spread and alpha of each ring, nearest first; the hue is the snake's own `ok`. */
const RINGS: [(u32, u8); 3] = [(1, 0x70), (3, 0x38), (5, 0x18)];

/// How far a modal's drop shadow spreads, in logical pixels.
const SHADE_SPREAD: u32 = 10;

// The compositor has no blur pass, so a halo is spelled out one ring at a
// time: three concentric strokes at falling alpha around the same shape.
pub fn bloom(fb: &mut PaintBuffer, r: Rect, radius: u32) {
    let (ok, s) = (roles().ok.argb(), scale());
    for (spread, alpha) in RINGS {
        let (argb, spread) = (with_alpha(ok, alpha), s.px(spread));
        let x = r.x.saturating_sub(spread);
        let y = r.y.saturating_sub(spread);
        fb.stroke_round(x, y, r.w + spread * 2, r.h + spread * 2, radius + spread, s.hair(), argb);
    }
}

// Everything under a modal is already painted, so the dim has to blend.
pub fn scrim(fb: &mut PaintBuffer) {
    fb.blend_rect(0, 0, fb.width, fb.height, roles().scrim.argb());
}

/// The drop shadow under a modal drawn as a shared card at `r`, rounded as the
/// card is.
pub fn shade(fb: &mut PaintBuffer, r: Rect) {
    let s = scale();
    let (radius, spread) = (s.px(CARD_RADIUS), s.px(SHADE_SPREAD));
    fb.shadow_round(r.x, r.y, r.w, r.h, radius, spread, roles().shadow.argb());
}
