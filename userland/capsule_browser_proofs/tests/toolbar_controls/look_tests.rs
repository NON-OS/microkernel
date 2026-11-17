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

use capsule_browser_proofs::browser::shell::route::Route;
use capsule_browser_proofs::browser::toolbar::badge;
use capsule_browser_proofs::browser::toolbar::look::{icon, lead, Shows};
use capsule_browser_proofs::browser::ui::toolbar_geom::Part;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::Controls;

use super::kit::{at_scale, changed, geom, live, paint};

fn only_in(part: Part, a: &Shows, b: &Shows, q: u32) {
    let _g = at_scale(q);
    let mut c = Controls::acting_on_press();
    let (one, two) = (paint(a, &mut c), paint(b, &mut c));
    let diff = changed(&one, &two);
    let here = diff.iter().all(|&(x, y)| c.at(x, y) == Some(part));
    assert!(!diff.is_empty() && here, "{part:?} at {q}");
}

#[test]
fn reload_turns_to_stop_and_the_star_fills_in_their_own_box() {
    let (idle, mut busy, mut kept, mut open) = (live(), live(), live(), live());
    (busy.loading, kept.marked, open.reading) = (true, true, true);
    assert_eq!(icon(Part::Reload, &idle), Some(IconId::Reload));
    assert_eq!(icon(Part::Reload, &busy), Some(IconId::Close));
    assert_eq!(icon(Part::Star, &idle), Some(IconId::Star));
    assert_eq!(icon(Part::Star, &kept), Some(IconId::StarFilled));
    assert_eq!((lead(true), lead(false)), (IconId::Lock, IconId::Search));
    for q in [4, 5, 6, 8] {
        only_in(Part::Reload, &idle, &busy, q);
        only_in(Part::Star, &idle, &kept, q);
        only_in(Part::Reader, &idle, &open, q);
    }
}

#[test]
fn every_badge_fits_the_slot_the_longest_one_fills() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let slot = geom().route;
        let bodies = Route::ALL.map(|r| badge::body(slot, r));
        for b in bodies {
            let (x, w) = (b.x as i32, b.w as i32);
            assert!(x >= slot.x && x + w == slot.right() && b.h as i32 <= slot.h, "{b:?} at {q}");
        }
        assert_eq!(bodies.iter().map(|b| b.w as i32).max(), Some(slot.w), "at {q}");
    }
}
