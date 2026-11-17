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

//! The scrollback search as a `Field` holding the query over the prompt row,
//! and in the hint bar beside it a `Label` saying what Enter does, with
//! `case` in the accent while case counts. The painter and the proofs read
//! the same boxes.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Field, Label, Rect};

use crate::term::select::Find;

pub const PLACEHOLDER: &str = "find";
pub const CASE: &str = "case";

/// The field, the hint and, while case counts, the mark that says so.
pub struct View {
    pub field: Rect,
    pub hint: Rect,
    pub case: Option<Rect>,
}

/// The field spans the prompt row and the hint bar under it, half the row
/// across from `SPACE.md` in, centred down the two. The hint takes the hint
/// bar right of it, and the case mark that hint's right end.
pub fn view(input: Rect, footer: Rect, case: bool) -> View {
    let s = scale();
    let (pad, gap) = (s.px(u32::from(SPACE.md)), s.px(u32::from(SPACE.sm)));
    let band = input.h.saturating_add(footer.h);
    let (fw, fh) = Field::new("").size();
    let across = input.w.saturating_sub(pad.saturating_mul(2));
    let w = (across / 2).max(fw.min(across));
    let h = fh.min(band);
    let y = input.y.saturating_add((band - h) / 2);
    let field = Rect { x: input.x.saturating_add(pad), y, w, h };
    let x = field.x.saturating_add(w).saturating_add(gap);
    let right = input.x.saturating_add(input.w).saturating_sub(pad);
    let room = Rect { x, y: footer.y, w: right.saturating_sub(x), h: footer.h };
    let cw = if case { Label::new(CASE).size().0.min(room.w) } else { 0 };
    let mark = Rect { x: right.saturating_sub(cw), w: cw, ..room };
    let hint = Rect { w: room.w.saturating_sub(cw).saturating_sub(gap.min(cw)), ..room };
    View { field, hint, case: case.then_some(mark) }
}

/// What Enter does with the query as it stands.
pub fn hint(f: &Find) -> &'static str {
    match (f.query.is_empty(), f.hit.is_some()) {
        (true, _) => "type to search history",
        (false, true) => "Enter older, Shift+Enter newer, Esc close",
        (false, false) => "no match",
    }
}
