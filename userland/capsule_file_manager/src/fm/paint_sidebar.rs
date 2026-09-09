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

use super::icon_draw::draw;
use super::icon_path::Icon;
use super::layout::{PAD_X, SIDEBAR_W, SIDE_FIRST_Y, SIDE_ROW_H};
use super::screen::Screen;
use super::sidebar_model::{SideHit, DOWNLOADS};
use super::sidebar_rows::{row_active, side_rows};
use super::state::State;
use super::theme::{CY, CY_DIM, DEEP, INK, INK2, INK3, LINE, RAISE};

// Fixed quick-access locations. `(display, path)`; paths match how the vfs
// reports directory prefixes.
pub const PLACES: [(&str, &str); 3] =
    [("Root", "/"), ("Documents", "/docs/"), ("Capsules", "/capsules/")];

// The active row's pill is inset from both edges and sits a touch shorter than
// its row, so consecutive active rows would never touch.
const PILL_X: u32 = 8;
const PILL_H: u32 = SIDE_ROW_H - 4;
const ICON_S: u32 = 18;

pub fn paint_sidebar(state: &State, fb: &mut PaintBuffer) {
    let h = fb.height;
    fb.fill_rect(0, 0, SIDEBAR_W, h, DEEP);
    fb.fill_rect(SIDEBAR_W - 1, 0, 1, h, LINE);
    let _ = fb.text_ttf(PAD_X as i32, 32, "NØNOS Files", INK, 21.0);
    for row in side_rows(state) {
        match &row.hit {
            None => section_label(fb, &row.label, row.y),
            Some(hit) => nav_row(state, fb, &row.label, row.y, hit),
        }
    }
}

// Section labels sit in the tertiary ink at the readable floor; anything smaller
// is clamped back up to it by the font layer anyway.
fn section_label(fb: &mut PaintBuffer, label: &str, y: u32) {
    let _ = fb.text_ttf(PAD_X as i32, (y + 4) as i32, label, INK3, 13.0);
}

// The icons are stroked line art, so a row only needs its tint; nothing is
// hollowed against the colour drawn underneath any more.
fn nav_row(state: &State, fb: &mut PaintBuffer, label: &str, y: u32, hit: &SideHit) {
    let active = row_active(state, hit);
    if active {
        fb.fill_round(PILL_X, y, SIDEBAR_W - PILL_X * 2, PILL_H, 11, RAISE);
        fb.fill_round(PILL_X, y, 3, PILL_H, 1, CY);
    }
    let tint = if active { CY_DIM } else { INK3 };
    let iy = y + (PILL_H - ICON_S) / 2;
    draw(fb, nav_icon(hit), PAD_X, iy, ICON_S, tint);
    let ink = if active { INK } else { INK2 };
    let _ = fb.text_ttf((PAD_X + 30) as i32, (y + 4) as i32, label, ink, 18.0);
}

/// The glyph a navigation row wears. Downloads is a directory but reads as a
/// destination, so it keeps the arrow rather than the generic folder.
fn nav_icon(hit: &SideHit) -> Icon {
    match hit {
        SideHit::Screen(Screen::Home) => Icon::Home,
        SideHit::Screen(Screen::Recents) => Icon::Clock,
        SideHit::Screen(Screen::Shared) => Icon::People,
        SideHit::Screen(Screen::Tags) => Icon::Tag,
        SideHit::Screen(Screen::Search) => Icon::Magnifier,
        SideHit::Screen(_) => Icon::Folder,
        SideHit::Path(p) if p == DOWNLOADS => Icon::Download,
        SideHit::Path(_) => Icon::Folder,
    }
}

/// Superseded by `sidebar_rows::side_hit`, which accounts for the variable
/// Favorites section. Kept for the callers still on the fixed-index geometry.
pub fn place_at(y: u32) -> Option<&'static str> {
    if y < SIDE_FIRST_Y.saturating_sub(6) {
        return None;
    }
    let idx = ((y - SIDE_FIRST_Y.saturating_sub(6)) / SIDE_ROW_H) as usize;
    PLACES.get(idx).map(|(_, p)| *p)
}
