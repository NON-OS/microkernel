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
//! Scrolled cuts a rounded shape at the buffer's top edge instead of drawing a
//! new one there: scrolled by `s`, the picture is the unscrolled one moved up
//! `s` rows, so a card cut by the edge gains no corners and no top border.

use nonos_toolkit::widgets::{Card, Pill, Rect, Scrolled, Tone};

use super::canvas::Canvas;
use super::serial::serial;

const AT: Rect = Rect { x: 10, y: 0, w: 200, h: 90 };

fn content(fb: &mut Scrolled<'_, '_>) {
    Card::new().title("Network").note("Grüße — 日本語").draw(fb, AT);
    Pill::new("Granted").tone(Tone::Ok).dot().draw(fb, Rect { x: 30, y: 8, w: 90, h: 26 });
}

#[test]
fn a_shape_scrolled_off_the_top_is_cut_not_redrawn() {
    let _g = serial();
    let whole = Canvas::new(AT, 0).painted(|fb| content(&mut Scrolled::new(fb, 0)));
    for scroll in [1u32, 4, 9, 12, 20, 40, 89] {
        let cut = Canvas::new(AT, 0).painted(|fb| content(&mut Scrolled::new(fb, scroll)));
        for y in 0..whole.h - scroll {
            for x in 0..whole.w {
                let (got, want) = (cut.get(x, y), whole.get(x, y + scroll));
                assert_eq!(got, want, "scroll {scroll}: ({x}, {y}) is {got:08X}, not {want:08X}");
            }
        }
    }
}
