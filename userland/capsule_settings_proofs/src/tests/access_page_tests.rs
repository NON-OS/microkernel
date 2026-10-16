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

//! The Accessibility page: five rows, stopped on in order; each knob held in
//! its range; the compositor's layout both ways; unread says so.

use crate::settings::access::knobs::{step, AccessPage, Knobs, MAG_STEPS};
use crate::settings::access::text::value;
use crate::settings::access::wire::{ask, body, knobs};
use crate::settings::schema::access_knob::Knob;
use crate::settings::schema::{field_count, slot_at, Slot};
use crate::settings::section::Section;

#[test]
fn the_rows_are_stops_in_order() {
    let s = Section::Accessibility;
    assert_eq!(field_count(s), 5);
    let order = [Knob::Output, Knob::Filter, Knob::Contrast, Knob::Magnify, Knob::Follow];
    for (i, k) in order.iter().enumerate() {
        assert!(slot_at(s, i) == Some(Slot::Access(*k)), "row {i}");
    }
}

#[test]
fn each_knob_steps_inside_its_range() {
    let mut p = AccessPage::default();
    assert!(!step(&mut p, Knob::Filter, -1), "already the first");
    for want in 1..5 {
        assert!(step(&mut p, Knob::Filter, 1));
        assert_eq!(p.knobs.filter, want);
    }
    assert!(!step(&mut p, Knob::Filter, 1), "tritan is the last");
    for want in MAG_STEPS.iter().skip(1) {
        assert!(step(&mut p, Knob::Magnify, 1));
        assert_eq!(p.knobs.magnify, *want);
    }
    assert!(!step(&mut p, Knob::Magnify, 1));
    assert!(step(&mut p, Knob::Output, 9) && p.output == 3, "four displays");
    assert!(step(&mut p, Knob::Contrast, 5) && p.knobs.contrast == 1);
}

#[test]
fn bodies_round_trip_and_bad_answers_are_refused() {
    let k = Knobs { filter: 3, contrast: 1, magnify: 12, follow: 1 };
    let b = body(2, &k);
    assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 2);
    assert_eq!(knobs(&b[4..]), Some(k), "the answer's layout is the body's after the display");
    let mut bad = b;
    bad[12..16].copy_from_slice(&5u32.to_le_bytes());
    assert_eq!(knobs(&bad[4..]), None, "a magnification step that does not exist");
    assert_eq!(knobs(&b[4..19]), None, "short");
    assert_eq!(ask(4), None);
}

#[test]
fn an_unread_display_says_so() {
    let p = AccessPage::default();
    assert_eq!(value(&p, Knob::Filter), "not read");
    assert_eq!(value(&p, Knob::Output), "Display 1");
    let read = AccessPage { read: true, ..p };
    assert_eq!((value(&read, Knob::Magnify), value(&read, Knob::Follow)), ("Off", "Pointer"));
}
