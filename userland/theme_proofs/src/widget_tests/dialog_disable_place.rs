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

//! Dialog: disabling one button changes nothing about where things are.
//! The dialog keeps its size and panel, every other button is drawn lit in
//! the same rect, so its body and focus ring sit where they did, and `hit`
//! names the same button at every probe, but `None` over the disabled one.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Dialog, Rect};

use super::boxes::boxes;
use super::dialog::dialogs;
use super::dialog_armed_place::unlit;
use super::dialog_hit::probes;
use super::dialog_row::drawn;
use super::serial::{serial, SCALES};

/// A line when disabling button `k` of `d` moves its size, panel or another
/// drawn button, or a hit other than its own, or leaves its own hit.
fn moved(d: Dialog, k: usize, at: Rect) -> Option<String> {
    let off = d.disable(k);
    let n = (0..8).take_while(|&i| drawn(unlit(d), i, at).is_some()).count();
    let rects = |d: Dialog| (0..n).map(|i| drawn(unlit(d), i, at)).collect::<Vec<_>>();
    let (plain, less) = (rects(d), rects(off));
    let kept = (0..n).all(|i| i == k || plain[i] == less[i]);
    let probes = probes(at, &plain);
    let want = |x, y| d.hit(at, x, y).filter(|&i| i != k);
    let hits = probes.iter().all(|&(x, y)| off.hit(at, x, y) == want(x, y));
    let over = k >= n || probes.iter().any(|&(x, y)| d.hit(at, x, y) == Some(k));
    let ok = d.size() == off.size() && d.panel(at) == off.panel(at) && kept && hits && over;
    (!ok).then(|| format!("plain {plain:?} with {k} off {less:?}, hits {hits}, over {over}"))
}

#[test]
fn dialog_disabling_a_button_moves_no_box_ring_or_other_hit() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, d) in dialogs() {
            for at in boxes(d.size()) {
                for k in 0..4 {
                    let e = moved(d, k, at);
                    fails.extend(e.map(|e| format!("{name} at {q}/4 in {at:?}: {e}")));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
