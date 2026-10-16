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
//! holder dropped, a ninth refused rather than one dropped; and only a
//! holder of that kind's capture bit may change it.

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
fn only_a_holder_of_that_kinds_capture_bit_may_change_it() {
    assert_eq!(
        (needs(KIND_CAMERA), needs(KIND_MICROPHONE), needs(3)),
        (Some(1 << 37), Some(1 << 38), None)
    );
    let holds = |want: u64| move |pid: u32, mask: u64| pid == 40 && mask == want;
    assert!(may_indicate(40, KIND_CAMERA, holds(1 << 37)));
    assert!(
        !may_indicate(40, KIND_MICROPHONE, holds(1 << 37)),
        "camera's bit is not the microphone's"
    );
    assert!(!may_indicate(41, KIND_CAMERA, holds(1 << 37)), "another process, on or off");
    assert!(!may_indicate(40, 3, |_, _| true), "an unknown kind");
    assert!(!may_indicate(0, KIND_CAMERA, |_, _| true));
}
