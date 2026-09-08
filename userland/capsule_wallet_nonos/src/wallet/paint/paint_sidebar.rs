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

use super::scale;
use nonos_app_skeleton::PaintBuffer;

use super::nav_icon::{self, Nav};
use super::ui;
use crate::wallet::state::{
    State, VIEW_HOME, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED,
};
use crate::wallet::theme::{ACCENT, DIM, FG, GREEN, GREEN_INK, INK, LINE2, MUTED, SEL};

// Nav geometry shared with the pointer hit-test (event::on_pointer) so drawing
// and click-mapping cannot drift. Item i sits at NAV_Y0 + i * NAV_STEP.
pub const NAV_X: u32 = 14;
pub const NAV_W: u32 = 172;
pub const NAV_H: u32 = 38;
pub const NAV_Y0: u32 = 96;
pub const NAV_STEP: u32 = 46;

pub fn paint_sidebar(state: &State, fb: &mut PaintBuffer) {
    // The real mark, not a coloured square. `logo` rasterises the brand SVG
    // and has been available in this module the whole time; the sidebar drew a
    // 20 by 20 accent rectangle in its place, which is what a placeholder
    // looks like when nobody comes back to it.
    super::logo::logo(fb, 16, 44, 26);
    let _ = fb.text_ttf(48, 48, "NONOS", FG(), scale::TITLE);
    let items = [
        (Nav::Home, "Home", VIEW_HOME),
        (Nav::Receive, "Receive", VIEW_RECEIVE),
        (Nav::Send, "Send", VIEW_SEND),
        (Nav::Proof, "Proof", VIEW_PROOF),
        (Nav::Shielded, "Shielded", VIEW_SHIELDED),
        (Nav::Token, "NOX", VIEW_NOX),
    ];
    for (i, (icon, label, view)) in items.into_iter().enumerate() {
        nav(fb, NAV_Y0 + i as u32 * NAV_STEP, icon, label, state.view == view);
    }

    let _ = fb.text_ttf(22, 700, "RAILS", DIM(), scale::BODY);
    ui::chip(fb, 22, 722, b"ETH", ACCENT(), INK());
    ui::chip(fb, 70, 722, b"NOX", GREEN(), GREEN_INK());
    ui::chip(fb, 118, 722, b"PR", LINE2(), MUTED());
}

fn nav(fb: &mut PaintBuffer, y: u32, kind: Nav, label: &str, active: bool) {
    let tint = if active { ACCENT() } else { LINE2() };
    if active {
        fb.fill_rect(NAV_X, y, NAV_W, NAV_H, SEL());
        fb.fill_rect(NAV_X, y, 3, NAV_H, ACCENT());
    }
    // Was one twelve-pixel square repeated six times down the rail, which told
    // a reader nothing except that an icon was meant to be there.
    nav_icon::icon(fb, NAV_X + 16, y + 12, kind, tint);
    let color = if active { FG() } else { MUTED() };
    let _ = fb.text_ttf((NAV_X + 38) as i32, (y + 10) as i32, label, color, scale::BODY);
}
