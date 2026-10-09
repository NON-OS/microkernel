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

//! One hit-test for the whole surface. Every branch calls the same geometry
//! function the painter used, so a click can only resolve to a control that
//! was actually drawn under the pointer.

use super::control::Control;
use super::geometry::{page, shell};
use super::hit_screen::{bar_hit, content_hit};
use super::shell as chrome;
use super::state::{UiState, View};

pub enum Action {
    Go(View),
    Ctl(Control),
    Select(usize),
    LibTab(usize),
    ClearQuery,
    /// Download the address in the Search field, and play it.
    Download,
    /// A Downloads row's button.
    Fetched(u32, crate::fetch::list::Act),
    ClearDownloads,
}

/// What a click can land on besides the window's fixed chrome.
pub struct Lists<'a> {
    /// The rows the current page shows, as library indices.
    pub rows: &'a [usize],
    pub queue: &'a [usize],
    /// Tracks in the library.
    pub n: usize,
    pub downloads: &'a [crate::fetch::list::Row],
}

pub fn hit(ui: &UiState, dims: (u32, u32), l: &Lists, x: i32, y: i32) -> Option<Action> {
    let sh = shell(dims.0, dims.1);
    if sh.transport.contains(x, y) {
        return bar_hit(sh.transport, x, y);
    }
    if sh.sidebar.contains(x, y) {
        return chrome::nav_at(sh.sidebar, x, y).map(Action::Go);
    }
    if sh.rail.contains(x, y) {
        if let Some(c) = chrome::rail_control_at(sh.rail, x, y) {
            return Some(Action::Ctl(c));
        }
        // The track the row shows, not the row number: under shuffle the
        // queue's order is not the library's.
        return chrome::rail_queue_track_at(sh.rail, l.queue, x, y).map(Action::Select);
    }
    if chrome::search_clear(sh.topbar).contains(x, y) && !ui.query.is_empty() {
        return Some(Action::ClearQuery);
    }
    if chrome::search_field(sh.topbar).contains(x, y) {
        return Some(Action::Go(View::Search));
    }
    content_hit(ui, page(&sh), l, x, y)
}
