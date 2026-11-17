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

//! Tabs: icons and busy flags left empty draw, size and place every tab
//! exactly as a plain strip does.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::Tabs;

use super::boxes::boxes;
use super::canvas::Canvas;
use super::serial::{serial, SCALES};
use super::tabs::{LABELS, LONGS};

#[test]
fn tabs_left_without_leads_draw_size_and_hit_as_plain() {
    let _g = serial();
    for q in SCALES {
        set_scale(q);
        for labels in [&LABELS[..], &LONGS[..]] {
            let plain = Tabs::new(labels, 1).hovered(Some(0));
            let same = plain.icons(&[]).busy(&[false; 3]);
            assert_eq!(same.size(), plain.size(), "size at {q}/4");
            for at in boxes(plain.size()) {
                let draw = |t: Tabs| Canvas::new(at, 0).painted(|fb| t.draw(fb, at)).px;
                assert!(draw(same) == draw(plain), "pixels in {at:?} at {q}/4");
                let tabs = |t: Tabs| (0..4).map(|i| t.tab(at, i)).collect::<Vec<_>>();
                assert_eq!(tabs(same), tabs(plain), "tabs in {at:?} at {q}/4");
            }
        }
    }
    set_scale(SCALES[0]);
}
