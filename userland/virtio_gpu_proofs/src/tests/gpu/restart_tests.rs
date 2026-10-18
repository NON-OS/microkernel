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
//! A driver restarted onto its own device. L78 resets the function
//! between holders and the bring-up resets it again, so the new driver
//! meets a device with no resource and no scanout set, whatever its killed
//! self was doing. Killed at every command of a draw and a mode set, the
//! next one takes every scanout whole, zeroed, from DMA it holds, and
//! draws.

use std::rc::Rc;

use nonos_gfx_wire::ops;

use super::ask::{ask, words};
use super::kit::{backed_live, draw, shown};
use super::rig::{rig, rig_on, Rig};
use super::wrong::{run, Wrong};

const OLD: u32 = 0x00AB_CDEF;
const NEW: u32 = 0x0013_5790;

fn last_moments(r: &mut Rig) {
    draw(r, 0, OLD);
    ask(r, ops::MODE_SET, 0, &words(&[800, 600]));
    draw(r, 0, OLD);
}

#[test]
fn killed_at_any_command_the_next_driver_comes_up_whole() {
    let n = {
        let mut r = rig();
        let from = r.host.borrow().steps();
        last_moments(&mut r);
        let to = r.host.borrow().steps();
        to - from
    };
    assert!(n >= 10, "{n} commands");
    for k in 1..=n + 1 {
        let mut r = rig();
        run(&mut r, Wrong::Killed(k), last_moments);
        let host = Rc::clone(&r.host);
        drop(r);
        host.borrow_mut().reset_device();
        let mut r = rig_on(host);
        let px = shown(&r, 0).expect("taken again");
        assert!(px.len() == 640 * 480 && px.iter().all(|&p| p == 0), "k {k}: zeroed, whole");
        assert!(backed_live(&r), "k {k}: the host reads only pages the new driver holds");
        assert_eq!(r.host.borrow().res.len(), 1, "k {k}: one surface");
        draw(&mut r, 0, NEW);
        assert!(shown(&r, 0).unwrap().iter().all(|&p| p == NEW), "k {k}: it draws");
    }
}
