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

//! Brightness on Appearance is shown and never set: each display's level
//! as the compositor answers it, exact to the percent, "not offered" for a
//! display without one, and no key on the row changes anything.

use super::step_tests::two;
use crate::settings::schema::access_knob::Knob;
use crate::settings::screens::arrange::step;
use crate::settings::screens::levels::{levels, ANSWER_LEN};
use crate::settings::screens::percent::percent_of;
use crate::settings::screens::text::value;

fn answer(count: u32, panels: &[(u32, u32)]) -> [u8; ANSWER_LEN] {
    let mut a = [0u8; ANSWER_LEN];
    a[..4].copy_from_slice(&count.to_le_bytes());
    for (i, (id, l)) in panels.iter().enumerate().take(3) {
        a[4 + i * 8..8 + i * 8].copy_from_slice(&id.to_le_bytes());
        a[8 + i * 8..12 + i * 8].copy_from_slice(&l.to_le_bytes());
    }
    a
}

#[test]
fn each_display_shows_its_own_level() {
    let got = levels(&answer(3, &[(0x8000_0001, 40), (0x8000_0000, 100), (0x8000_0002, 255)]));
    assert_eq!(got, [Some(100), Some(40), None, None], "255 is unknown");
    let lying =
        levels(&answer(u32::MAX, &[(0x8000_0101, 5), (0x8000_0003, 101), (0x8000_0003, 7)]));
    assert_eq!(lying, [None, None, None, Some(7)], "no output past four, no level past 100");
    assert_eq!(levels(&answer(0, &[(0x8000_0000, 50)])), [None; 4], "a count of none");
}

#[test]
fn a_level_reads_exactly_and_the_row_never_changes_it() {
    for v in 0..=100u8 {
        assert_eq!(percent_of(v), format!("{v}%"));
    }
    assert_eq!(percent_of(255), "100%");
    let mut p = two();
    p.levels = [Some(60), None, None, None];
    assert_eq!(value(&p, Knob::Brightness), "60%");
    p.output = 1;
    assert_eq!(value(&p, Knob::Brightness), "not offered");
    for delta in [-64, -1, 1, 64] {
        assert!(!step(&mut p, Knob::Brightness, delta), "read only");
    }
    p.read = false;
    assert_eq!(value(&p, Knob::Brightness), "not read");
}
