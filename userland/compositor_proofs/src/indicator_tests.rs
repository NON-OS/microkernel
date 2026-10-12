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

//! The capture indicator's holders and who may change them: lit while any
//! capturer holds it, one turning off leaving another's light on, a dead
//! holder dropped, a ninth refused rather than one dropped; and no sender
//! at all while the kernel's capture bits are not yet named.

use crate::display::indicator_rule::{may_indicate, needs};
use crate::state::indicator::{Indicators, Refused, KIND_CAMERA, KIND_MICROPHONE};

#[test]
fn the_light_stays_on_while_any_capturer_holds_it() {
    let mut i = Indicators::default();
    assert_eq!(i.apply(KIND_CAMERA, true, 40), Ok(true), "lit");
    assert_eq!(i.apply(KIND_CAMERA, true, 41), Ok(false), "already lit");
    assert_eq!(i.apply(KIND_CAMERA, false, 40), Ok(false), "41 still captures");
    assert!(i.shown(KIND_CAMERA) && !i.shown(KIND_MICROPHONE));
    assert_eq!(i.apply(KIND_CAMERA, false, 41), Ok(true), "dark");
    assert_eq!(i.apply(KIND_CAMERA, false, 99), Ok(false), "off from no holder changes nothing");
    assert_eq!(i.apply(7, true, 40), Err(Refused::Kind));
}

#[test]
fn a_ninth_capturer_is_refused_and_a_dead_one_dropped() {
    let mut i = Indicators::default();
    for pid in 1..=8 {
        i.apply(KIND_MICROPHONE, true, pid).unwrap();
    }
    assert_eq!(i.apply(KIND_MICROPHONE, true, 9), Err(Refused::Full));
    assert!(!i.reap(|p| p != 3), "seven still capture");
    assert!(i.reap(|_| false), "all gone: dark");
    assert!(!i.shown(KIND_MICROPHONE));
}

#[test]
fn nobody_may_light_it_before_the_capture_bits_are_named() {
    assert_eq!((needs(KIND_CAMERA), needs(KIND_MICROPHONE), needs(3)), (None, None, None));
    assert!(!may_indicate(40, KIND_CAMERA, |_, _| true), "even a process holding every bit");
    assert!(!may_indicate(0, KIND_MICROPHONE, |_, _| true));
}
