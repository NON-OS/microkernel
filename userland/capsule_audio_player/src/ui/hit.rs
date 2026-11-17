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

//! One hit-test for the whole surface. A click names the control under it
//! from the target table, which the hover reads too and which is built from
//! the geometry the painters use, so a click can only resolve to a control
//! that was actually drawn under the pointer.

use super::control::Control;
use super::geometry::shell;
use super::hit_screen::action;
use super::screen::any_finished;
use super::shown::{target_at, Shown};
use super::state::{UiState, View};
use crate::fetch::row_text::label;

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

/// What the window shows, for the target table.
pub fn shown(ui: &UiState, dims: (u32, u32), l: &Lists) -> Shown {
    let acts = l.downloads.iter().map(|r| r.acts().iter().map(|&a| label(a)).collect());
    Shown {
        view: ui.view,
        dims,
        scroll: ui.scroll,
        query: !ui.query.is_empty(),
        address: crate::fetch::is_address(&ui.query),
        rows: l.rows.len(),
        tracks: l.n,
        queue: l.queue.len(),
        acts: acts.collect(),
        finished: any_finished(l.downloads),
    }
}

pub fn hit(ui: &UiState, dims: (u32, u32), l: &Lists, x: i32, y: i32) -> Option<Action> {
    let sh = shell(dims.0, dims.1);
    target_at(&shown(ui, dims, l), x, y).and_then(|t| action(t, ui, &sh, l, x, y))
}
