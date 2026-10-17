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
//! A surface the host would not let go of waits in limbo with its pages
//! mapped (the host may still read them), its id never handed out again,
//! and is given back on the next mode set once the host answers.

use nonos_gfx_wire::ops;

use super::ask::{ask, words};
use super::kit::{backed_live, draw, live};
use super::rig::rig;
use super::wrong::{run, Wrong};
use crate::constants::VG_CMD_RESOURCE_UNREF;

#[test]
fn a_surface_the_host_keeps_waits_in_limbo_then_goes() {
    // Where a clean mode set unreferences the old surface (found on a rig
    // of its own first: rigs share the thread's hook and DMA).
    let at = {
        let mut p = rig();
        draw(&mut p, 0, 1);
        let from = p.host.borrow().steps();
        ask(&mut p, ops::MODE_SET, 0, &words(&[800, 600]));
        let log = &p.host.borrow().log[from as usize..];
        log.iter().position(|c| c.kind == VG_CMD_RESOURCE_UNREF).expect("an unref") as u32
    };
    let mut r = rig();
    draw(&mut r, 0, 0x0011_2233);
    let old = r.s.display.heads[0].as_ref().unwrap().shown.id;
    let set =
        run(&mut r, Wrong::Dead(at + 1, 1), |r| ask(r, ops::MODE_SET, 0, &words(&[800, 600])));
    assert_eq!(set.unwrap().0, 0, "the mode set itself landed");
    assert_eq!(r.s.display.limbo.iter().map(|s| s.id).collect::<Vec<_>>(), vec![old]);
    assert!(r.host.borrow().res.contains_key(&old) && backed_live(&r), "pages still mapped");
    r.s.display.next_id = old;
    assert!(r.s.display.fresh_id() != old, "its id never handed out");
    let held = live().len();
    assert_eq!(ask(&mut r, ops::MODE_SET, 0, &words(&[1024, 768])).0, -16, "RETIRE first");
    let serial = r.s.slots[0].retiring.unwrap().serial;
    assert_eq!(ask(&mut r, ops::RETIRE, 0, &words(&[serial])).0, 0);
    assert_eq!(ask(&mut r, ops::MODE_SET, 0, &words(&[1024, 768])).0, 0);
    assert!(r.s.display.limbo.is_empty() && !r.host.borrow().res.contains_key(&old));
    assert_eq!(live().len(), held - 1, "its pages unmapped at last");
}
