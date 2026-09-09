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
use nonos_libc::mk_time_millis;

use super::card::{card, Card};
use super::file_color::color;
use super::file_kind::kind_of_name;
use super::home_geom::{cards_bottom, cards_label_y, home_lines};
use super::home_storage::storage_cards;
use super::layout::{CONTENT_X, FOOTER_H, HEADER_H, PAD_X};
use super::recents_group::parent_of;
use super::screen_row::{section_label, LABEL_ADV};
use super::sidebar_rows::base_label;
use super::state::State;
use super::theme::{INK, INK3};

pub fn paint_home(state: &State, fb: &mut PaintBuffer) {
    let x = CONTENT_X + PAD_X;
    let w = fb.width.saturating_sub(CONTENT_X + PAD_X * 2);
    let bottom = fb.height.saturating_sub(FOOTER_H);
    let now = mk_time_millis().max(0) as u64;
    let _ = fb.text_ttf(x as i32, (HEADER_H + 16) as i32, greeting(now), INK, 30.0);
    section_label(fb, x, cards_label_y(), "CONTINUE WORKING");
    recent_cards(state, fb, x, w, now);
    storage_cards(state, fb, x, cards_bottom(state, now), w, bottom);
}

// The wall clock is the only time source, and `fmt_time` already reads a civil
// time-of-day out of it the same way, so a UTC hour is a sound derivation here.
fn greeting(now_ms: u64) -> &'static str {
    match now_ms / 1000 % 86_400 / 3600 {
        0..=11 => "Good morning.",
        12..=17 => "Good afternoon.",
        _ => "Good evening.",
    }
}

// Draws the cards `home_geom` laid out; an empty journal says so in their place.
fn recent_cards(state: &State, fb: &mut PaintBuffer, x: u32, w: u32, now: u64) {
    let lines = home_lines(state, now);
    if lines.is_empty() {
        let y = cards_label_y() + LABEL_ADV;
        let _ = fb.text_ttf(x as i32, y as i32, "Nothing opened yet.", INK3, 15.0);
        return;
    }
    for line in &lines {
        let path = line.path.as_str();
        let title = base_label(path);
        let spec = Card {
            title: title.as_str(),
            sub: parent_of(path),
            meta: line.meta.as_str(),
            tint: color(kind_of_name(path)),
            dir: line.dir,
            bar: None,
        };
        card(fb, x, line.y, w, &spec);
    }
}
