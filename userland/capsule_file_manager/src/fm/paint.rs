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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Rect};

use super::help::paint_help;
use super::layout::{hair, metrics};
use super::paint_footer::paint_footer;
use super::paint_grid::paint_grid;
use super::paint_header::paint_header;
use super::paint_info::paint_info;
use super::paint_rows::paint_rows;
use super::preview_paint::paint_preview;
use super::screen::Screen;
use super::screen_home::paint_home;
use super::screen_recents::paint_recents;
use super::screen_search::paint_search;
use super::screen_tags::paint_tags;
use super::sel_band::paint_band;
use super::state::{Mode, State, ViewKind};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if matches!(state.mode, Mode::Help) {
        paint_help(fb);
        return;
    }
    if let (Mode::Preview, Some(preview)) = (state.mode, state.preview.as_ref()) {
        paint_preview(preview, fb);
        return;
    }
    let (w, h) = (fb.width, fb.height);
    let (shell, r, radius) = (Rect { x: 0, y: 0, w, h }, roles(), metrics().r_shell);
    fb.clear(0);
    draw::round(fb, shell, radius, r.bg);
    draw::outline(fb, shell, radius, hair(), r.border);
    super::paint_sidebar::paint_sidebar(state, fb);
    paint_header(state, fb);
    match state.screen {
        Screen::Browse => paint_browse(state, fb),
        Screen::Home => paint_home(state, fb),
        Screen::Recents => paint_recents(state, fb),
        Screen::Search => paint_search(state, fb),
        Screen::Tags => paint_tags(state, fb),
    }
    paint_footer(state, fb);
}

// The directory listing plus the info panel describing what the cursor is on.
// The presentation toggle applies only to the listing half; the panel's strip is
// reserved out of the content width, so the two never overlap.
fn paint_browse(state: &State, fb: &mut PaintBuffer) {
    match state.view {
        ViewKind::Grid => paint_grid(state, fb),
        ViewKind::List => paint_rows(state, fb),
    }
    paint_info(state, fb);
    paint_band(state, fb);
}
