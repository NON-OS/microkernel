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

//! Dialog: disabling one button changes only the pixels inside it, and
//! some when it is drawn live; inside it the dialog draws what a disabled
//! dialog draws there; an index past the last changes no pixel, and a
//! disabled dialog draws the same with a button disabled or not.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Dialog, Rect, State};

use super::boxes::{boxes, inside};
use super::dialog::dialogs;
use super::dialog_armed_paint::painted;
use super::dialog_armed_place::unlit;
use super::dialog_row::drawn;
use super::serial::{serial, SCALES};

/// A line if disabling button `k` of `d` changes a pixel outside it, or
/// none though it is drawn, or draws inside it other than a disabled
/// dialog does, or changes the disabled dialog, or if a `k` past the last
/// changes anything.
fn strays(d: Dialog, k: usize, at: Rect) -> Option<String> {
    let dead = d.state(State { disabled: true, ..State::default() });
    let (plain, off, all) = (painted(d, at), painted(d.disable(k), at), painted(dead, at));
    let diff = plain.diff(&off);
    let own = drawn(unlit(d), k, at);
    let out = diff.iter().find(|&&pt| !own.is_some_and(|r| inside(r, pt)));
    let unlike = all.diff(&off).into_iter().filter(|&p| own.is_some_and(|r| inside(r, p)));
    let unlike = unlike.count();
    let gone = all.diff(&painted(dead.disable(k), at)).len();
    let past = plain.diff(&painted(d.disable(usize::MAX), at)).len();
    let bad = out.is_some() || unlike + gone + past > 0 || diff.is_empty() == own.is_some();
    let n = diff.len();
    bad.then(|| format!("{out:?} of {n} off {own:?}; {unlike} unlike, {gone} dead, {past} past"))
}

#[test]
fn dialog_disabling_a_button_repaints_only_that_button() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, d) in dialogs() {
            for at in boxes(d.size()) {
                for k in 0..4 {
                    let e = strays(d, k, at);
                    fails.extend(e.map(|e| format!("{name} at {q}/4 in {at:?}: {e}")));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
