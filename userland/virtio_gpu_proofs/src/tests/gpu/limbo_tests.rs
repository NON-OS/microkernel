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
//! A surface the host would not let go of (it refused the detach, so the
//! unref too) waits in limbo with its pages mapped (the host may still read
//! them), its id never handed out again, and is given back on the next mode
//! set once the host answers, or once a recovery reset the host.

use nonos_gfx_wire::ops;

use super::ask::{ask, words};
use super::kit::{backed_live, draw, live};
use super::recover_kit::stall;
use super::rig::rig;
use super::wrong::{run, Wrong};
use crate::constants::VG_CMD_RESOURCE_DETACH_BACKING as DETACH;

#[test]
fn a_surface_the_host_keeps_waits_in_limbo_then_goes() {
    let mut r = in_limbo();
    let old = r.d.server.display.limbo[0].id;
    r.d.server.display.next_id = old;
    assert!(r.d.server.display.fresh_id() != old, "its id never handed out");
    let held = live().len();
    assert_eq!(ask(&mut r, ops::MODE_SET, 0, &words(&[1024, 768])).0, -16, "RETIRE first");
    let serial = r.d.server.slots[0].retiring.unwrap().serial;
    assert_eq!(ask(&mut r, ops::RETIRE, 0, &words(&[serial])).0, 0);
    assert_eq!(ask(&mut r, ops::MODE_SET, 0, &words(&[1024, 768])).0, 0);
    assert!(r.d.server.display.limbo.is_empty() && !r.host.borrow().res.contains_key(&old));
    assert_eq!(live().len(), held - 1, "its pages unmapped at last");
}

/// A rig whose last mode set left the old surface in limbo: the host refused
/// its detach (found on a rig of its own first), and so its unref.
fn in_limbo() -> super::rig::Rig {
    let at = {
        let mut p = rig();
        draw(&mut p, 0, 1);
        let from = p.host.borrow().steps();
        ask(&mut p, ops::MODE_SET, 0, &words(&[800, 600]));
        let log = &p.host.borrow().log[from as usize..];
        log.iter().position(|c| c.kind == DETACH).expect("a detach") as u32
    };
    let mut r = rig();
    draw(&mut r, 0, 0x0011_2233);
    let old = r.d.server.display.heads[0].as_ref().unwrap().shown.id;
    let set =
        run(&mut r, Wrong::Refused(at + 1), |r| ask(r, ops::MODE_SET, 0, &words(&[800, 600])));
    assert_eq!(set.unwrap().0, 0, "the mode set itself landed");
    assert_eq!(r.d.server.display.limbo.iter().map(|s| s.id).collect::<Vec<_>>(), vec![old]);
    assert!(r.host.borrow().res.contains_key(&old) && backed_live(&r), "pages still mapped");
    r
}

#[test]
fn a_recovery_gives_limbo_back_as_the_reset_host_holds_none() {
    let mut r = in_limbo();
    let s = super::ask::surface(&mut r, 0);
    let held = live().len();
    assert!(!stall(&mut r, &s, 1), "recovered");
    assert!(r.d.server.display.limbo.is_empty(), "limbo emptied");
    assert_eq!(live().len(), held - 1, "its pages unmapped");
}
