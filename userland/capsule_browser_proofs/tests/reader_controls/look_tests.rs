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

use capsule_browser_proofs::browser::shell::kept::ReaderPrefs;
use capsule_browser_proofs::browser::ui::reader_hit::Control;
use capsule_browser_proofs::browser::ui::rect::Rect;
use nonos_toolkit::widgets::Controls;

use super::kit::{at_scale, changed, controls, geom, mid, paint, prefs, W};

#[test]
fn each_control_lights_only_in_its_own_box() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (g, mut c) = (geom(), Controls::acting_on_press());
        let unlit = paint(W, &prefs(), &mut c);
        for (ctl, r) in controls(&g) {
            let (x, y) = mid(r);
            c.moved(x, y);
            let diff = changed(&unlit, &paint(W, &prefs(), &mut c));
            let swatch = matches!(ctl, Control::Theme(_));
            assert!(swatch || !diff.is_empty(), "{ctl:?} does not light at {q}");
            assert!(diff.iter().all(|&(x, y)| r.contains(x, y)), "{ctl:?} at {q}");
            c.left();
        }
    }
}

/* Every pixel that moves between `a` and `b` lies in one of `on`. */
fn moves_only(a: &ReaderPrefs, b: &ReaderPrefs, on: &[Rect], q: u32) {
    let mut c = Controls::acting_on_press();
    let diff = changed(&paint(W, a, &mut c), &paint(W, b, &mut c));
    assert!(!diff.is_empty(), "{on:?} at {q}");
    assert!(diff.iter().all(|&(x, y)| on.iter().any(|r| r.contains(x, y))), "{on:?} at {q}");
}

#[test]
fn a_new_choice_changes_only_its_own_controls() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (g, p) = (geom(), prefs());
        moves_only(&p, &ReaderPrefs { theme: 2, ..p }, &[g.swatches[0], g.swatches[2]], q);
        moves_only(&p, &ReaderPrefs { width: 0, ..p }, &[g.widths[0], g.widths[1]], q);
        moves_only(&p, &ReaderPrefs { show_time: true, ..p }, &[g.time], q);
        moves_only(&p, &ReaderPrefs { size: 3, ..p }, &[g.dots], q);
    }
}
