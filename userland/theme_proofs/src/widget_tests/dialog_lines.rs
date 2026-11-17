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

//! Dialog: `body_lines` lets a body the user must read whole take more
//! than `DIALOG_LINES` lines. Unasked, a dialog is the one that asks for
//! `DIALOG_LINES`: the same size, panel, cut, text in the same places and
//! the same hits, in its own box, wider and narrower ones and short ones.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::design::size::DIALOG_LINES;
use nonos_toolkit::widgets::{Dialog, Rect};

use super::boxes::{boxed, boxes};
use super::dialog::dialogs;
use super::dialog_hit::probes;
use super::dialog_spans::spans;
use super::serial::{serial, SCALES};

/// A capability list far longer than `DIALOG_LINES` lines.
pub(crate) fn list() -> String {
    (0..48).map(|i| format!("Capability{i}")).collect::<Vec<_>>().join(", ")
}

/// A line when `d` and `d` asking for `DIALOG_LINES` differ in `at`.
fn unlike(d: Dialog, at: Rect) -> Option<String> {
    let a = d.body_lines(DIALOG_LINES);
    let hits = probes(at, &[]).into_iter().all(|(x, y)| d.hit(at, x, y) == a.hit(at, x, y));
    let same = d.size() == a.size() && d.panel(at) == a.panel(at) && d.cut(at) == a.cut(at);
    (!(same && hits && spans(d, at) == spans(a, at))).then(|| format!("{at:?}"))
}

#[test]
fn dialog_unasked_takes_dialog_lines() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, d) in dialogs() {
            let (w, h) = d.size();
            for at in boxes((w, h)).into_iter().chain([boxed(w, h / 2), boxed(w, h - 1)]) {
                fails.extend(unlike(d, at).map(|e| format!("{name} at {q}/4: {e}")));
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
