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
use nonos_toolkit::theme::roles;

use super::super::metrics::metrics;

// The real brand mark, from the same 512x556 rasterization of
// wallet_logos/nonos-icon-teal.svg that the wallet and the dock draw. This card
// used to approximate it with a ring and a diagonal line: close enough to read as
// the logo at a glance, and wrong in every detail next to the actual one. An
// About box that shows an imitation of its own product's mark is the last place
// that should be guessing.
const ICON: &[u8] = include_bytes!("../../../../../assets/icons/nonos_logo.rgba");
const ICON_W: u32 = 512;
const ICON_H: u32 = 556;

// The keyline the mark sits in. It survives from the drawn version because it
// gives the hero a left edge to align against, and it is a frame rather than a
// depiction of anything.
const RING_GAP: u32 = 7;

/// The mark's radius, and how far its ring sits in from the card's padding, in
/// logical pixels.
const HERO_MARK_R: u32 = 30;
const MARK_INSET: u32 = 8;

pub fn mark(fb: &mut PaintBuffer, y: i32) {
    let (r, m, s) = (roles(), metrics(), scale());
    let (mark_r, ring_r, hair) = (s.px(HERO_MARK_R), s.px(HERO_MARK_R) + s.px(RING_GAP), s.hair());
    let cy = y + (m.hero_h / 2) as i32;
    let reach = (ring_r + hair) as i32;
    if cy < reach || cy + reach >= fb.height as i32 {
        return;
    }
    let cx = m.card_pad + mark_r + s.px(MARK_INSET);
    let cy = cy as u32;
    fb.ring(cx, cy, ring_r, hair, r.border_strong.argb());

    /*
     * Fitted to the ring's inner diameter and centred on the same point, with the
     * mark's own 512:556 aspect kept: scaling it to a square would flatten the
     * logo, which is the sort of small wrongness this change exists to remove.
     */
    let box_h = mark_r * 2;
    let dh = box_h;
    let dw = (box_h * ICON_W / ICON_H).max(1);
    fb.blit_rgba8_scaled(cx - dw / 2, cy - dh / 2, dw, dh, ICON, ICON_W, ICON_H);
}
