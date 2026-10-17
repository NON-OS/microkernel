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
//! The blank's making gone wrong at each of its commands: none is left in
//! the table, its DMA is given back, and a host that answered the clean-up
//! keeps no resource of it. And a driver with no blank still takes a dead
//! owner's frame off: its scanouts are left detached, the screen dark.

use nonos_libc::dma_live;

use super::draw::{draw, SECRET};
use super::kit::{run, Wrong};
use super::rig::rig;
use crate::death::watch::watch;
use crate::setup::blank_surface;

#[test]
fn a_blank_that_cannot_be_made_is_given_back_whole() {
    for k in 1..=3 {
        for w in [Wrong::Dead(k, 0), Wrong::Dead(k, 1)] {
            let r = rig();
            let (held, next) = (dma_live(), r.d.resources.alloc_id());
            let d = &r.d;
            let p = d.primary.unwrap();
            let ids = (d.device_id, d.claim_epoch);
            let made = run(&r, w, || {
                blank_surface::create(ids, &d.control_queue, &d.fences, &d.resources, &p)
            });
            assert!(made.expect("not killed").is_none(), "{w:?}: none");
            assert_eq!(dma_live(), held, "{w:?}: its DMA given back");
            assert!(d.resources.lookup(next + 1).is_none(), "{w:?}: not in the table");
            let kept = r.gpu.borrow().res.contains_key(&(next + 1));
            assert!(!kept || matches!(w, Wrong::Dead(_, 0)), "{w:?}: nor on the host");
        }
    }
}

#[test]
fn with_no_blank_a_dead_owner_s_scanouts_are_left_detached() {
    let mut r = rig();
    r.d.blank = None;
    draw(&r);
    assert!(watch(&r.d, 100, false));
    let g = r.gpu.borrow();
    assert_eq!((g.scanouts[0], g.scanouts[1]), (0, 0), "dark");
    assert!(g.res.values().all(|x| !x.px.contains(&SECRET)));
    assert!(r.d.owner.quiet.get());
}
