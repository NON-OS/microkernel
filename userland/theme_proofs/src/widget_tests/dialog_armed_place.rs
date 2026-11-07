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

//! Dialog: arming changes nothing about where things are. An armed dialog
//! has the same size and panel as the unarmed one, each button is drawn
//! lit in the same rect, so its body and focus ring sit where they did,
//! and `hit` names the same button at every probe, disabled or live.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Dialog, Rect, State};

use super::boxes::boxes;
use super::dialog::dialogs;
use super::dialog_hit::probes;
use super::dialog_row::drawn;
use super::serial::{serial, SCALES};

/// `d` with no button focused, hovered or held.
pub(crate) fn unlit(d: Dialog) -> Dialog {
    d.hovered(None).pressed(None).focus(usize::MAX)
}

/// A line when arming `d` moves its size, panel, a drawn button or a hit.
fn moved(d: Dialog, at: Rect) -> Option<String> {
    let a = d.armed(true);
    let n = (0..8).take_while(|&i| drawn(unlit(d), i, at).is_some()).count();
    let rects = |d: Dialog| (0..n).map(|i| drawn(unlit(d), i, at)).collect::<Vec<_>>();
    let (plain, armed) = (rects(d), rects(a));
    let off = State { disabled: true, ..State::default() };
    let same = |d: Dialog, a: Dialog, (x, y): (i32, i32)| d.hit(at, x, y) == a.hit(at, x, y);
    let both = |p| same(d, a, p) && same(d.state(off), a.state(off), p);
    let hits = probes(at, &plain).into_iter().all(both);
    let ok = d.size() == a.size() && d.panel(at) == a.panel(at) && plain == armed && hits;
    (!ok).then(|| format!("plain {plain:?} armed {armed:?}, hits agree {hits}"))
}

#[test]
fn dialog_arming_moves_no_box_ring_or_hit() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, d) in dialogs() {
            for at in boxes(d.size()) {
                fails.extend(moved(d, at).map(|e| format!("{name} at {q}/4 in {at:?}: {e}")));
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
