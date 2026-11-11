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

//! The pane header: a title and a line saying what the lists below hold.
//!
//! It greeted a named person ("Welcome back, Mehedi") on every machine, and
//! drew a search field that took no input and a "Create New" card of
//! templates (Report, Letter, Resume, Project Plan) that did nothing. The
//! editor has no accounts, no document index and no templates; a new document
//! is File > New Tab or Ctrl+N.

use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::truncate;

use super::metrics::{body, head, pane_pad};
use super::metrics_pane::{pane_content, strap_y};

const TITLE_TEXT: &str = "Documents";
const STRAPLINE: &str = "Open a file from the store, or one this session opened.";

pub(super) fn paint_pane_head(fb: &mut PaintBuffer) {
    let ((x, w), r) = (pane_content(fb.width), roles());
    let title = truncate(TITLE_TEXT, w, head());
    let _ = fb.text_ttf(x as i32, pane_pad() as i32, &title, r.text.argb(), head());
    let sub_y = strap_y() as i32;
    let sub = truncate(STRAPLINE, w, body());
    let _ = fb.text_ttf(x as i32, sub_y, &sub, r.muted.argb(), body());
}
