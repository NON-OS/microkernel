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

//! The outcome mark a finished command gets: a tick or a cross, drawn.

use nonos_app_skeleton::PaintBuffer;

use super::{GAP, MARK};
use crate::term::block::Status;
use crate::term::theme::types::Theme;

/// The outcome, as strokes rather than a character, and the left edge it
/// leaves behind.
///
/// The UI face carries no tick and no cross, so asking it for one drew a
/// missing-glyph box. Two lines say the same thing and are legible at a size
/// no font in the tree renders well at. A running command has no outcome yet
/// and gets nothing, which is why the column is empty while it works.
pub(super) fn status_mark(fb: &mut PaintBuffer, s: Status, right: i32, top: i32, t: &Theme) -> i32 {
    let x = right - MARK;
    let mid = top + MARK / 2;
    match s {
        Status::Running => return right,
        Status::Ok => {
            for d in 0..2 {
                let pts = [
                    (x, mid + d),
                    (x + MARK / 3, mid + MARK / 3 + d),
                    (x + MARK, mid - MARK / 2 + d),
                ];
                fb.polyline_aa(&pts, t.ok);
            }
        }
        Status::Err => {
            fb.line_aa(x, mid - MARK / 2, x + MARK, mid + MARK / 2, t.err);
            fb.line_aa(x, mid + MARK / 2, x + MARK, mid - MARK / 2, t.err);
        }
    }
    x - GAP
}
