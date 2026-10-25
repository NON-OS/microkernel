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
//! MODE_SET: a new surface of the size holding the primary whole, set on
//! the scanout, and the old one given back (unreferenced on the host, its
//! DMA unmapped). One the host refuses keeps the old mode and leaves
//! nothing of the new behind.

use nonos_gfx_wire::{ops, Surface};

use super::ask::{ask, words};
use super::kit::{backed_live, draw, fill, live, shown};
use super::rig::rig;
use super::wrong::{run, Wrong};

#[test]
fn a_mode_set_shows_the_new_size_and_gives_the_old_surface_back() {
    let mut r = rig();
    let old = draw(&mut r, 0, 0x0011_2233);
    let (before, ids) = (live().len(), r.host.borrow().res.len());
    let set = ask(&mut r, ops::MODE_SET, 0, &words(&[800, 600]));
    assert_eq!(set.0, 0);
    let new = Surface::decode(&set.1).unwrap();
    assert_eq!((new.width, new.height), (800, 600));
    let px = shown(&r, 0).unwrap();
    assert!(px.len() == 800 * 600 && px.iter().all(|&p| p == 0), "the new primary, zero");
    assert_eq!((live().len(), r.host.borrow().res.len()), (before, ids), "one for one");
    assert!(backed_live(&r));
    assert_eq!(ask(&mut r, ops::RETIRE, 0, &words(&[old.serial])).0, 0);
    fill(&mut r, 0, 0x0044_5566);
    assert_eq!(ask(&mut r, ops::FLUSH, 0, &words(&[new.serial, 1, 0, 0, 800, 600])).0, 0);
    assert!(shown(&r, 0).unwrap().iter().all(|&p| p == 0x0044_5566));
}

#[test]
fn a_mode_set_the_host_refuses_keeps_the_old_mode_and_leaves_nothing() {
    let mut r = rig();
    draw(&mut r, 0, 0x0011_2233);
    let (before, ids) = (live(), r.host.borrow().res.len());
    let set = run(&mut r, Wrong::Dead(2, 1), |r| ask(r, ops::MODE_SET, 0, &words(&[800, 600])));
    assert!(set.unwrap().0 < 0, "refused");
    let px = shown(&r, 0).unwrap();
    assert!(px.len() == 640 * 480 && px.iter().all(|&p| p == 0x0011_2233), "the old mode");
    assert_eq!((live(), r.host.borrow().res.len()), (before, ids), "nothing left");
}
