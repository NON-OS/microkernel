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

use capsule_browser_proofs::browser::ui::settings_geom::Control;
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{at_scale, changed, controls, geom, live, mid, paint};

/// The controls the page draws for itself: the fields.
fn page_drawn(c: Control) -> bool {
    matches!(c, Control::Field(_))
}

#[test]
fn each_control_lights_and_acts_only_where_it_is_drawn() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (g, s) = (geom(), live());
        for (ctl, r) in controls(&g) {
            let (mut c, (x, y)) = (Controls::acting_on_press(), mid(r));
            let unlit = paint(&g, &s, &mut c);
            c.moved(x, y);
            let lit = paint(&g, &s, &mut c);
            assert_eq!(c.pressed(x, y), Step::Act(ctl), "{ctl:?} at {q}");
            let pressed = paint(&g, &s, &mut c);
            for diff in [changed(&unlit, &lit), changed(&unlit, &pressed)] {
                assert!(diff.iter().all(|&(x, y)| c.at(x, y) == Some(ctl)), "{ctl:?} at {q}");
                assert!(page_drawn(ctl) || !diff.is_empty(), "{ctl:?} at {q}");
            }
            c.released();
            c.left();
            assert_eq!(paint(&g, &s, &mut c), unlit, "{ctl:?} once left, at {q}");
        }
    }
}

#[test]
fn a_control_with_nothing_to_act_on_never_lights_or_acts() {
    let dim = [Control::ProxyOff, Control::ClearHistory, Control::ClearDownloads, Control::Smaller];
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let g = geom();
        let s = Default::default();
        let mut c = Controls::acting_on_press();
        let unlit = paint(&g, &s, &mut c);
        for (ctl, r) in controls(&g) {
            let (x, y) = mid(r);
            let gone = dim.contains(&ctl) || ctl == Control::RemoveBookmarks;
            assert_eq!(c.at(x, y).is_none(), gone, "{ctl:?} at {q}");
            if gone {
                c.moved(x, y);
                assert_eq!(c.pressed(x, y), Step::Idle, "{ctl:?} at {q}");
                assert_eq!(paint(&g, &s, &mut c), unlit, "{ctl:?} at {q}");
                c.released();
            }
        }
    }
}
