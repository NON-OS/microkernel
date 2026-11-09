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

use capsule_browser_proofs::browser::pages::pills::row;
use capsule_browser_proofs::browser::shell::groups::MAX_GROUPS;
use capsule_browser_proofs::browser::shell::kept::Group;
use capsule_browser_proofs::browser::ui::page_part::{PagePart, Pick};
use nonos_toolkit::widgets::{Controls, Step};

use super::fixtures::tabs;
use super::kit::{at_scale, changed, geom, mid, paint, spots};

#[test]
fn new_group_is_dead_once_the_window_is_full() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let mut s = tabs(false);
        s.groups =
            (1..=MAX_GROUPS as u8).map(|id| Group { id, name: "G".into(), color: id }).collect();
        let mut c = Controls::acting_on_press();
        let unlit = paint(&s, &mut c);
        assert!(spots(&c).iter().all(|(p, _)| *p != PagePart::Pill(Pick::NewGroup)), "{q}");
        let g = geom(&s);
        let (specs, widths) = row(&s, &g);
        assert_eq!(specs.last().map(|p| (p.pick, p.dim)), Some((Pick::NewGroup, true)), "{q}");
        let (x, y) = mid(g.pill(&widths, widths.len() - 1));
        c.moved(x, y);
        assert_eq!(c.pressed(x, y), Step::Idle, "{q}");
        assert!(changed(&unlit, &paint(&s, &mut c)).is_empty(), "{q}");
    }
}
