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
//! FLUSH: the rectangles named reach the host's copy and the screen, the
//! rest stays as it was; a host that drops the transfer answers ETIMEDOUT
//! and shows nothing of the rectangle, and the turn's recovery shows the
//! frame the surface's pages hold, whole.

use nonos_gfx_wire::errno::ETIMEDOUT;
use nonos_gfx_wire::ops;

use super::ask::{ask, send, words};
use super::kit::{draw, fill, shown};
use super::rig::rig;
use super::wrong::{run, Wrong};
use crate::constants::VG_CMD_RESOURCE_FLUSH as RESOURCE_FLUSH;
use crate::server::recover::settle;

#[test]
fn a_flush_shows_its_rectangle_and_nothing_else() {
    let mut r = rig();
    let s = draw(&mut r, 0, 0x0011_2233);
    fill(&mut r, 0, 0x00AB_CDEF);
    let from = r.host.borrow().steps() as usize;
    assert_eq!(ask(&mut r, ops::FLUSH, 0, &words(&[s.serial, 1, 10, 20, 30, 40])).0, 0);
    let flushed = r.host.borrow().log[from..].iter().any(|c| c.landed && c.kind == RESOURCE_FLUSH);
    assert!(flushed, "flushed to the screen, not only sent");
    let px = shown(&r, 0).unwrap();
    for (i, &p) in px.iter().enumerate() {
        let (x, y) = (i as u32 % 640, i as u32 / 640);
        let inside = (10..40).contains(&x) && (20..60).contains(&y);
        assert_eq!(p, if inside { 0x00AB_CDEF } else { 0x0011_2233 }, "({x}, {y})");
    }
}

#[test]
fn a_host_that_drops_the_transfer_answers_timed_out_and_shows_none_of_it() {
    let mut r = rig();
    let s = draw(&mut r, 0, 0x0011_2233);
    fill(&mut r, 0, 0x00AB_CDEF);
    let all = words(&[s.serial, 1, 0, 0, 640, 480]);
    let got = run(&mut r, Wrong::Dead(1, 1), |r| send(r, ops::FLUSH, 0, &all).0);
    assert_eq!(got, Some(ETIMEDOUT));
    assert!(shown(&r, 0).unwrap().iter().all(|&p| p == 0x0011_2233), "the old picture");
    // The stall reset the device; the next turn shows the pages whole.
    assert!(!settle(&mut r.d), "recovered");
    assert!(shown(&r, 0).unwrap().iter().all(|&p| p == 0x00AB_CDEF), "the new, whole");
    assert_eq!(ask(&mut r, ops::FLUSH, 0, &all).0, 0, "served again");
}
