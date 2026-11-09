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

//! Menu: a heading names the group under it. It is never hit at any point,
//! the arrow keys step over it, and a grouped menu stays in its box in every
//! state at every scale.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Item, Menu};

use super::bounds::assert_bounded;
use super::boxes::boxes;
use super::menu_tag_hit::around;
use super::serial::{serial, SCALES};

/// Two groups of options, as an HTML select with `<optgroup>`s lists them,
/// one option disabled, the second group after a separator.
pub(crate) const GROUPED: [Item; 7] = [
    Item::heading("Früchte"),
    Item::new("Apfel").indent(),
    Item::new("Birne — 日本語").indent().checked(true),
    Item::separator(),
    Item::heading("Gemüse"),
    Item::new("Lauch").indent().disabled(),
    Item::new("Möhre").indent().shortcut("M"),
];

/// The rows of `GROUPED` that are headings.
pub(crate) const HEADS: [usize; 2] = [0, 4];

#[test]
fn menu_heading_is_never_hit_and_never_reached() {
    let _g = serial();
    for q in SCALES {
        set_scale(q);
        let m = Menu::new(&GROUPED);
        for at in boxes(m.size()) {
            let hit = around(at).find_map(|(x, y)| m.hit(at, x, y).filter(|i| HEADS.contains(i)));
            assert_eq!(hit, None, "in {at:?} at {q}/4");
        }
    }
    set_scale(SCALES[0]);
    let froms = (0..=GROUPED.len()).map(Some).chain([None]);
    for (from, step) in froms.flat_map(|f| (-4..=4).map(move |s| (f, s))) {
        let to = Menu::next(&GROUPED, from, step);
        assert!(to.is_none_or(|i| !HEADS.contains(&i)), "{from:?} by {step}: {to:?}");
    }
    assert_eq!(Menu::next(&GROUPED, None, 1), Some(1));
    assert_eq!(Menu::next(&GROUPED, Some(2), 1), Some(6));
    assert_eq!(Menu::next(&[Item::heading("Leer")], None, 1), None);
}

#[test]
fn menu_heading_paints_inside_its_box() {
    let _g = serial();
    let lit = [(None, None), (Some(0), Some(0)), (Some(4), Some(1)), (Some(1), None)];
    let menu = |(h, p)| Menu::new(&GROUPED).hovered(h).pressed(p);
    assert_bounded(&lit.map(|l| (format!("grouped lit {l:?}"), menu(l))));
}
