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

//! The calculator's controls at the display scale: at 1x, 1.25x, 1.5x and 2x
//! every control of every mode sits inside a window grown by the same scale,
//! no two overlap, and what a click names is the box its painter draws. Its
//! own binary, so setting the scale races no other test.

use apps_proofs::calc::hit::Hit;
use apps_proofs::calc::manifest::{HEIGHT, WIDTH};
use apps_proofs::calc::mode::{Mode, MODES};
use apps_proofs::calc::state::State;
use apps_proofs::calc::ui::target::{at, targets};
use apps_proofs::calc::ui::{history_geom, keypad_geom, nav_geom};
use nonos_toolkit::design::scale::{scale, set_scale};

fn windows(w: i32, h: i32) -> Vec<State> {
    let mut out = Vec::new();
    for (mode, n) in MODES.map(|m| (m, 0)).into_iter().chain([(Mode::History, 32)]) {
        let mut s = State::new();
        s.set_mode(mode);
        (0..n).for_each(|_| s.history.push(b"1 + 1", 2));
        s.view = (w, h);
        out.push(s);
    }
    out
}

fn every_control_fits_names_itself_and_is_the_box_painted(w: i32, h: i32) {
    for s in windows(w, h) {
        let ts = targets(&s);
        for (i, &(t, r)) in ts.iter().enumerate() {
            let (x1, y1) = ((r.x + r.w) as i32, (r.y + r.h) as i32);
            assert!(r.w > 0 && x1 <= w && y1 <= h, "{r:?} leaves {w}x{h} in {}", s.mode.label());
            let mid = ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32);
            assert!(at(&s, mid.0, mid.1) == Some(t) && at(&s, x1 - 1, y1 - 1) == Some(t));
            for &(_, b) in &ts[i + 1..] {
                let beside = r.x + r.w <= b.x || b.x + b.w <= r.x;
                assert!(beside || r.y + r.h <= b.y || b.y + b.h <= r.y, "{r:?} meets {b:?}");
            }
        }
        let keys = keypad_geom::keys(s.mode, w, h).map(|(row, i, _, r)| (Hit::Key(row, i), r));
        let rails = MODES.iter().enumerate().map(|(i, m)| (Hit::Rail(*m), nav_geom::row(i)));
        for (hit, drawn) in rails.chain(keys) {
            assert!(ts.iter().any(|&(t, r)| t == hit && r == drawn), "{drawn:?} is not hit");
        }
    }
    assert!(history_geom::capacity(h) > 0);
}

#[test]
fn every_control_names_what_is_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let s = scale();
        every_control_fits_names_itself_and_is_the_box_painted(
            s.px(WIDTH) as i32,
            s.px(HEIGHT) as i32,
        );
    }
    set_scale(4);
}
