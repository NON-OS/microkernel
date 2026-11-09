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

use capsule_browser_proofs::browser::ui::reader_geom::{MARGIN, MIN_W};
use capsule_browser_proofs::browser::ui::reader_hit::Control;
use nonos_toolkit::widgets::Controls;

use super::kit::{at_scale, geom, mid, paint, prefs, W};

#[test]
fn the_whole_time_row_works_the_switch_and_the_dots_do_nothing() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let (g, mut c) = (geom(), Controls::acting_on_press());
        paint(W, &prefs(), &mut c);
        assert_eq!(c.at(g.time.x + 2, g.time.y + 2), Some(Control::Time), "{q}");
        assert_eq!(c.at(g.time.right() - 2, g.time.bottom() - 2), Some(Control::Time), "{q}");
        let (x, y) = mid(g.dots);
        assert_eq!(c.at(x, y), None, "{q}");
    }
}

#[test]
fn a_card_too_narrow_to_draw_places_nothing() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let w = (MIN_W + 2 * MARGIN) * q as i32 / 4 - 1;
        let mut c = Controls::acting_on_press();
        assert!(paint(w, &prefs(), &mut c).iter().all(|&p| p == 0), "{q}");
        assert!((0..w).all(|x| (0..400).all(|y| c.at(x, y).is_none())), "{q}");
    }
}
