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
use nonos_toolkit::widgets::text::{left, truncate, width};
use nonos_toolkit::widgets::{Pill, Rect, Tone};

use crate::about::data::admission::{self, Admission};
use crate::about::data::{abi, build, product};
use crate::about::format::trimmed;

use super::super::card;
use super::super::metrics::metrics;
use super::super::mono::valid;
use super::overview_mark::mark;

/// Where the hero's text starts, the tops of its three lines, and the gap
/// under the badge, in logical pixels.
const HERO_TEXT_X: u32 = 96;
const HERO_TITLE_TOP: u32 = 20;
const HERO_SUB_TOP: u32 = 58;
const HERO_META_TOP: u32 = 84;
const BADGE_GAP: u32 = 10;

// The identity band: the mark, the product, and the four facts that name this
// exact image. The badge on the right is a passive label, not a control: it
// says how the kernel's spawn gate admitted this window, as the attestation
// registry records it, and is green only for an admission under a proof.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32, admission: Admission) {
    let (r, m, s) = (roles(), metrics(), scale());
    let x = s.px(HERO_TEXT_X);
    card::panel(fb, 0, y, w, m.hero_h);
    mark(fb, y);
    left(fb, x, y + s.px(HERO_TITLE_TOP) as i32, valid(product::NAME), r.text, m.value_px);
    let sub = y + s.px(HERO_SUB_TOP) as i32;
    let cut = truncate(valid(product::TAGLINE), w.saturating_sub(x + m.card_pad), m.body_px);
    left(fb, x, sub, &cut, r.text_2, m.body_px);
    meta(fb, y + s.px(HERO_META_TOP) as i32);
    badge(fb, y, w, admission);
}

fn meta(fb: &mut PaintBuffer, y: i32) {
    let (r, px) = (roles(), metrics().body_px);
    let version = valid(trimmed(build::VERSION));
    let mut x = left(fb, scale().px(HERO_TEXT_X), y, version, r.text_2, px);
    for (value, accent) in [(build::GIT_SHA, false), (build::ARCH, false), (abi::NAME, true)] {
        x = left(fb, x.max(0) as u32, y, "   /   ", r.muted, px);
        let fg = if accent { r.accent } else { r.text_2 };
        x = left(fb, x.max(0) as u32, y, valid(value), fg, px);
    }
}

fn badge(fb: &mut PaintBuffer, y: i32, w: u32, admission: Admission) {
    let (r, m, s) = (roles(), metrics(), scale());
    let (label, pass, sub) = admission::badge(admission);
    let top = y + s.px(HERO_TITLE_TOP) as i32;
    let pill = Pill::new(valid(label)).tone(if pass { Tone::Ok } else { Tone::Danger }).dot();
    let (bw, bh) = pill.size();
    let x = w.saturating_sub(m.card_pad + bw);
    if top >= 0 && top + bh as i32 <= fb.height as i32 {
        pill.draw(fb, Rect { x, y: top as u32, w: bw, h: bh });
    }
    let sw = width(valid(sub), m.body_px);
    let sx = w.saturating_sub(m.card_pad + sw);
    left(fb, sx, top + (bh + s.px(BADGE_GAP)) as i32, valid(sub), r.muted, m.body_px);
}
