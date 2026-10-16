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

//! The displays' rows on Appearance: each steps inside its range, the scale
//! row for the display chosen, and says what the screens do.

use crate::settings::schema::access_knob::Knob;
use crate::settings::screens::arrange::{step, ScreenPage, Shown, SCALES};
use crate::settings::screens::text::value;

fn two() -> ScreenPage {
    let shown = Shown { outputs: 2, scale_q: 6, ..Shown::default() };
    ScreenPage { shown, output: 0, read: true }
}

#[test]
fn each_row_steps_inside_its_range() {
    let mut p = two();
    assert!(step(&mut p, Knob::Arrange, 1) && p.shown.extend);
    assert!(!step(&mut p, Knob::Arrange, 1), "extended is the last");
    assert!(step(&mut p, Knob::Screen, 9) && p.output == 1, "two displays, not four");
    for want in &SCALES[1..] {
        assert!(step(&mut p, Knob::Scale, 1));
        assert_eq!(p.shown.chosen, [0, *want, 0, 0], "the second display's alone");
    }
    assert!(!step(&mut p, Knob::Scale, 1), "4x is the last");
    assert!(step(&mut p, Knob::Scale, -64) && p.shown.chosen[1] == 0, "back to automatic");
}

#[test]
fn the_rows_say_what_the_screens_do() {
    let mut p = two();
    assert_eq!(value(&p, Knob::Arrange), "Mirrored");
    assert_eq!(value(&p, Knob::Scale), "Automatic, 150%", "the first display's scale now");
    p.shown.extend = true;
    assert_eq!(value(&p, Knob::Arrange), "Extended, on a second display", "chosen, not shown");
    p.shown.extended = true;
    assert_eq!(value(&p, Knob::Arrange), "Extended");
    (p.output, p.shown.chosen[1]) = (1, 8);
    assert_eq!((value(&p, Knob::Screen), value(&p, Knob::Scale)), ("Display 2", "200%"));
    p.read = false;
    assert_eq!(value(&p, Knob::Arrange), "not read");
}
