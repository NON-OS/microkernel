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

//! The claims this window cannot settle, and why.
//!
//! This is the card that makes the other two worth reading. A verification
//! surface that lists only what it can prove is telling you a selected truth: the
//! reader has no way to know what was left out, and every system's About box
//! looks complete for exactly that reason.
//!
//! So the claims that cannot be tested from here are named, each with the reason
//! it is out of reach and the place it is actually settled. Two of them are out
//! of reach precisely because this capsule is confined, which is the system
//! working rather than failing.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left, truncate};

use super::super::card::{self, titled};
use super::super::kv::row_h;
use super::super::metrics::metrics;
use super::super::mono::valid;
use super::prose;
use super::verify_items::ITEMS;

const ITEM_GAP: u32 = 8;

pub fn height(inner: u32) -> u32 {
    let (m, rh, item_gap) = (metrics(), row_h(), scale().px(ITEM_GAP));
    let body = inner.saturating_sub(m.card_pad);
    let mut h = card::overhead() + rh;
    for (claim, why) in ITEMS {
        let _ = claim;
        h += rh + prose::height(why, body) + item_gap;
    }
    h
}

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let (m, rh, item_gap) = (metrics(), row_h(), scale().px(ITEM_GAP));
    let r = roles();
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, height(inner), b"Not checkable from here");
    let note = "Named rather than omitted, so this list is not a selected truth.";
    left(fb, m.card_pad, top + centre_top(0, rh, m.body_px), note, r.muted, m.body_px);

    let body = inner.saturating_sub(m.card_pad);
    let mut row_y = top + rh as i32;
    for (claim, why) in ITEMS {
        let cut = truncate(valid(claim), inner, m.body_px);
        left(fb, m.card_pad, row_y + centre_top(0, rh, m.body_px), &cut, r.text_2, m.body_px);
        let why_y = row_y + rh as i32;
        prose::paint(fb, m.card_pad + m.card_pad, why_y, body, why, r.muted);
        row_y = why_y + (prose::height(why, body) + item_gap) as i32;
    }
}
