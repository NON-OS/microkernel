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
//! A scanout the host turns back on and off again before it was shown
//! (G-77): off once more, it answers dark and a MODE_SET waits, so no mode
//! is set on a screen the host does not show; back again, the primary is
//! shown whole. Another output never notices one turned off.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::errno::EAGAIN;
use nonos_gfx_wire::ops;

use super::ask::{ask, words};
use super::kit::{draw, raise, shown, state};
use super::rig::{rig, rig_with};
use crate::tests::host::words::PANEL;

const PX: u32 = 0x0021_4365;

#[test]
fn off_again_before_it_was_shown_is_dark_again() {
    let mut r = rig();
    draw(&mut r, 0, PX);
    for d in [(0, 0), PANEL, (0, 0)] {
        raise(&mut r, 0, d);
        assert!(r.d.server.display.events(), "{d:?} is a change");
    }
    assert_eq!(state(&mut r, 0).flags, 0, "dark again");
    assert_eq!(ask(&mut r, ops::MODE_SET, 0, &words(&[800, 600])).0, EAGAIN);
    assert!(shown(&r, 0).is_none(), "nothing set on a screen the host has off");
    raise(&mut r, 0, PANEL);
    assert!(r.d.server.display.events());
    assert_eq!(state(&mut r, 0).flags, LOST);
    let px = shown(&r, 0).expect("shown again");
    assert!(px.len() == 640 * 480 && px.iter().all(|&p| p == PX));
    assert_eq!(state(&mut r, 0).flags, SCANNING_OUT);
}

#[test]
fn another_output_never_notices() {
    let mut r = rig_with(&[PANEL, PANEL]);
    draw(&mut r, 0, PX);
    draw(&mut r, 1, PX);
    raise(&mut r, 1, (0, 0));
    assert!(r.d.server.display.events());
    assert_eq!((state(&mut r, 0).flags, state(&mut r, 1).flags), (SCANNING_OUT, 0));
    let px = shown(&r, 0).expect("output 0 still shows");
    assert!(px.iter().all(|&p| p == PX));
    assert!(shown(&r, 1).is_none(), "output 1 let go");
}
