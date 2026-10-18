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

//! A damaged canvas rectangle through the lens redraws exactly the screen
//! pixels whose centres fall in it, and one wholly left of the lens draws
//! nothing.

use crate::damage::Rect;
use crate::display::magnifier_rule::Lens;
use crate::tests::blit::magnify_tests::{shown, OUT, SH, SW};

#[test]
fn a_damaged_rectangle_redraws_exactly_what_it_covers() {
    let l = Lens { z: 10, ox: 4, oy: 2 };
    for rect in [Rect { x: 6, y: 3, width: 3, height: 2 }, Rect { x: 0, y: 0, width: 5, height: 3 }]
    {
        let (screen, _) = shown(rect, l);
        for sy in 0..SH {
            for sx in 0..SW {
                let (cx, cy) = (l.ox + (4 * sx + 2) / l.z, l.oy + (4 * sy + 2) / l.z);
                let covered = (rect.x..rect.x + rect.width).contains(&cx)
                    && (rect.y..rect.y + rect.height).contains(&cy);
                let got = screen[(sy * SW + sx) as usize];
                assert_eq!(got != OUT, covered, "at {sx},{sy}");
            }
        }
    }
    assert!(shown(Rect { x: 0, y: 0, width: 4, height: 2 }, l).1.is_none(), "left of the lens");
}
