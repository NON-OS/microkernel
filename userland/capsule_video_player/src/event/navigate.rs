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

use nonos_app_skeleton::app::EventOutcome;

use super::action::Action;
use super::hit::{line_step, page_step};
use crate::app::VideoApp;
use crate::ui::frame::region;
use crate::ui::rows::{scroll_for, wheel_scroll};
use crate::ui::screen::Route;
use crate::ui::view::render::content;

pub fn navigate(app: &mut VideoApp, action: Action) -> Option<EventOutcome> {
    match action {
        Action::ShowLibrary => Some(goto(app, Route::Library)),
        Action::Goto(route) => Some(goto(app, route)),
        Action::Back => {
            leaving_player(app);
            Some(outcome(app.nav.back()))
        }
        Action::MoveSel(delta) => Some(move_sel(app, delta)),
        Action::Scroll(delta_y) => Some(scroll(app, delta_y)),
        Action::OpenSelected => Some(open(app, app.browse.sel)),
        Action::OpenIndex(index) => Some(open(app, index)),
        Action::ToggleGrid => {
            app.browse.grid = !app.browse.grid;
            Some(EventOutcome::Repaint)
        }
        Action::Type(byte) => Some(outcome(app.browse.type_byte(byte))),
        Action::Erase => Some(outcome(app.browse.erase())),
        Action::Folder(folder) => Some(outcome(app.browse.set_folder(folder))),
        _ => None,
    }
}

/// Leaving the player keeps where the video stands, for the library's
/// watched bar and for opening it there again.
fn leaving_player(app: &mut VideoApp) {
    if app.route() == Route::Player {
        app.note_position();
    }
}

fn goto(app: &mut VideoApp, route: Route) -> EventOutcome {
    if route != Route::Player {
        leaving_player(app);
        app.playing = false;
    }
    // The folder filter belongs to the Folders page; Library always lists all.
    if route == Route::Library {
        app.browse.set_folder(None);
    }
    outcome(app.nav.go(route))
}

fn move_sel(app: &mut VideoApp, delta: i32) -> EventOutcome {
    let count = app.browse.len();
    if count == 0 {
        return EventOutcome::Idle;
    }
    let sel = (app.browse.sel as i64 + delta as i64).clamp(0, count as i64 - 1) as usize;
    app.browse.sel = sel;
    // The page the list is drawn on decides what is in sight; a page with no
    // list (Media Details) keeps the selection at the top for when it shows.
    let (w, h) = app.dims;
    let grid = app.browse.grid;
    let (visible, stride) = match content(app.route(), region::body(w, h)) {
        Some(area) => (page_step(area, grid), line_step(area, grid)),
        None => (1, 1),
    };
    app.browse.scroll = scroll_for(sel, app.browse.scroll, visible, stride);
    EventOutcome::Repaint
}

/* The wheel moved the selection a tile at a time, so the grid scrolled a
 * line only every few notches. It moves the page now, a line of tiles or
 * three rows a notch, and leaves the selection where it was. */
fn scroll(app: &mut VideoApp, delta_y: i32) -> EventOutcome {
    let (w, h) = app.dims;
    let grid = app.browse.grid;
    let Some(area) = content(app.route(), region::body(w, h)) else {
        return EventOutcome::Idle;
    };
    let (visible, stride) = (page_step(area, grid), line_step(area, grid));
    let next = wheel_scroll(app.browse.scroll, app.browse.len(), visible, stride, delta_y);
    outcome(core::mem::replace(&mut app.browse.scroll, next) != next)
}

fn open(app: &mut VideoApp, index: usize) -> EventOutcome {
    outcome(app.open_index(index))
}

fn outcome(changed: bool) -> EventOutcome {
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}
