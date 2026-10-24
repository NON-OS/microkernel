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
//! A touchscreen's first finger: pressed where it lands, dragged, and
//! released, followed by its identifier whatever slot it is in.

use super::digitizers::*;
use super::multitouch_tests::mt;
use crate::hid::ScreenEvent;

#[test]
fn a_screen_presses_drags_and_releases_its_first_finger() {
    let mut m = mt(SCREEN, 2);
    let mut step = |r: Vec<u8>| {
        let f = m.frame(&r).expect("a frame");
        m.screen.step(&f, (4000, 3000))
    };
    let p = step(report(&[at(true, 7, 4000, 1500), at(false, 0, 0, 0)], 1, false));
    assert_eq!(p, [Some(ScreenEvent::Press(0x7FFF, 0x3FFF)), None]);
    let d = step(report(&[at(true, 8, 0, 0), at(true, 7, 2000, 3000)], 2, false));
    assert_eq!(d, [Some(ScreenEvent::Move(0x3FFF, 0x7FFF)), None], "finger 7, wherever its slot");
    let swap = step(report(&[at(true, 8, 0, 0), at(false, 7, 0, 0)], 2, false));
    assert_eq!(swap, [Some(ScreenEvent::Release), Some(ScreenEvent::Press(0, 0))]);
    let up = step(report(&[at(false, 8, 0, 0), at(false, 0, 0, 0)], 1, false));
    assert_eq!(up, [Some(ScreenEvent::Release), None]);
}
