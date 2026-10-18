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
//! The multitouch path: every finger's fields found, a screen told from a
//! pad, hybrid frames put together, a parallel pad's first finger down
//! followed when the first slot lifts (screen_tests.rs has a screen's).

use super::digitizers::*;
use crate::hid::{primary, Multitouch, Surface};

pub(super) fn mt(kind: u8, fingers: usize) -> Multitouch {
    let d = digitizer(kind, fingers);
    Multitouch::new(&d)
}

#[test]
fn every_finger_is_found_and_a_screen_told_from_a_pad() {
    let (pad, screen, one) = (mt(PAD, 2), mt(SCREEN, 2), mt(PAD, 1));
    assert_eq!((pad.slots.n, pad.surface, pad.serves()), (2, Surface::Pad, true));
    assert_eq!((screen.surface, screen.serves()), (Surface::Screen, true));
    let s1 = pad.slots.list[1];
    assert_eq!((s1.id.bit_offset, s1.x.bit_offset, s1.y.bit_offset), (56, 64, 80));
    assert_eq!((pad.max(), one.serves()), ((4000, 3000), false), "one finger keeps its path");
    assert!(mt(SCREEN, 1).serves(), "a hybrid screen");
}

#[test]
fn a_parallel_pad_follows_the_finger_still_down() {
    let mut m = mt(PAD, 2);
    // Finger 0 lifts: counted once more, tip clear, ahead of finger 1.
    let f = m.frame(&report(&[at(false, 0, 0, 0), at(true, 1, 1200, 900)], 2, false)).unwrap();
    let s = primary(&f, m.max());
    assert_eq!((s.tip, s.x, s.y, s.contacts, s.contact_id), (true, 1200, 900, 1, Some(1)));
    let f = m.frame(&report(&[at(true, 0, 10, 20), at(true, 1, 30, 40)], 2, true)).unwrap();
    assert_eq!((primary(&f, m.max()).contacts, primary(&f, m.max()).button), (2, true));
    assert!(m.frame(&report(&[at(true, 0, 1, 1)], 1, false)).is_none(), "torn: one finger short");
    assert!(m.frame(&[5, 0, 0]).is_none(), "another report id");
}

#[test]
fn a_hybrid_frame_is_whole_on_its_last_report() {
    let mut m = mt(SCREEN, 1);
    assert!(m.frame(&report(&[at(true, 3, 100, 100)], 2, false)).is_none(), "one of two");
    let f = m.frame(&report(&[at(true, 4, 200, 200)], 0, false)).expect("both");
    assert_eq!((f.n, f.touches[0].id, f.touches[1].id), (2, 3, 4));
    assert!(m.frame(&report(&[at(true, 4, 1, 1)], 0, false)).is_none(), "no frame open");
}
