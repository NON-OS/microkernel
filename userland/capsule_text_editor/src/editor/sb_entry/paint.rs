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
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Field, Rect, State};

use super::{EntryOp, SbEntry};
use crate::editor::layout::{activity_w, row_h, sidebar_w};
use crate::editor::shell::pane_y;

// The input bar at the top of the sidebar: a focused field holding the name
// typed so far, with a caret at its end. A new file or folder shows its icon,
// and while the name is empty a hint says what it is for.
pub(in crate::editor) fn paint_entry(fb: &mut PaintBuffer, entry: &SbEntry, ribbon: bool) {
    let (hint, icon) = match entry.op {
        EntryOp::NewFile => ("New file name", Some(IconId::FsFile)),
        EntryOp::NewFolder => ("New folder name", Some(IconId::FsFolder)),
        EntryOp::Rename => ("New name", None),
    };
    let at = Rect { x: activity_w(), y: pane_y(ribbon), w: sidebar_w(), h: row_h() };
    let field = Field::new(&entry.buf).placeholder(hint);
    let field = icon.map_or(field, |id| field.icon(id));
    let focused = State { focused: true, ..State::default() };
    field.caret(Some(entry.buf.len())).state(focused).draw(fb, at);
}
