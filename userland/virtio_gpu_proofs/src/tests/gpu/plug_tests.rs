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
//! A scanout the host turns off and on again: off, it is let go of,
//! STATE answers dark, a FLUSH sends nothing and a MODE_SET waits
//! (EAGAIN); back, STATE answers LOST once while the wire server shows the
//! primary on it whole again, then scans out. An event that changes
//! nothing moves nothing.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::errno::EAGAIN;
use nonos_gfx_wire::ops;

use super::ask::{ask, words};
use super::kit::{backed_live, draw, fill, raise, shown, state};
use super::rig::rig;
use crate::constants::GPU_CFG_EVENTS_CLEAR;
use crate::tests::host::words::PANEL;
use crate::tests::model::DEVICE_AT;

const PX: u32 = 0x0044_5566;
const LATER: u32 = 0x0077_8899;

#[test]
fn off_goes_dark_and_back_shows_the_primary_whole() {
    let mut r = rig();
    let s = draw(&mut r, 0, PX);
    raise(&mut r, 0, (0, 0));
    assert!(r.d.server.display.events(), "turned off is a change");
    assert_eq!(r.host.borrow().scanouts[0], 0, "the scanout let go of the surface");
    assert_eq!(state(&mut r, 0).flags, 0, "dark");
    fill(&mut r, 0, LATER);
    let before = r.host.borrow().steps();
    let all = words(&[s.serial, 1, 0, 0, s.width, s.height]);
    assert_eq!(ask(&mut r, ops::FLUSH, 0, &all).0, 0);
    assert_eq!(r.host.borrow().steps(), before, "nothing sent while off");
    assert_eq!(ask(&mut r, ops::MODE_SET, 0, &words(&[800, 600])).0, EAGAIN);
    raise(&mut r, 0, PANEL);
    assert!(r.d.server.display.events(), "back is a change");
    assert_eq!(state(&mut r, 0).flags, LOST, "lost once, set again by the server");
    let px = shown(&r, 0).expect("shown again");
    assert!(px.len() == 640 * 480 && px.iter().all(|&p| p == LATER), "the primary whole");
    assert_eq!(state(&mut r, 0).flags, SCANNING_OUT);
    assert_eq!(r.host.borrow().res.len(), 1, "the old surface given back");
    assert!(r.d.server.display.limbo.is_empty() && backed_live(&r));
}

#[test]
fn an_event_that_changes_nothing_moves_nothing() {
    let mut r = rig();
    let g = state(&mut r, 0).generation;
    raise(&mut r, 0, PANEL);
    assert!(!r.d.server.display.events(), "no change");
    assert_eq!(state(&mut r, 0).generation, g);
    let clear = DEVICE_AT + GPU_CFG_EVENTS_CLEAR;
    assert_eq!(r.regs.wrote32(clear), 1, "the bit cleared all the same");
}
