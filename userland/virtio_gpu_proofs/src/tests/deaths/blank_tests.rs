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
//! The blank on a live host: what the screen and the host hold after it,
//! and the driver's word on its scanouts kept the host's, from bring-up on.

use super::ask::{ask, with64, words, OWNER};
use super::draw::{draw, fill, primary_px, SECRET};
use super::judge::clean;
use super::kit::agree;
use super::rig::rig;
use crate::death::watch::watch;
use crate::protocol::{OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_TRANSFER_TO_HOST};

#[test]
fn a_dead_owner_s_screen_is_blank_and_the_host_holds_none_of_its_frame() {
    let r = rig();
    let own = draw(&r);
    assert!(!watch(&r.d, 2_999, true), "not yet silent");
    assert!(watch(&r.d, 100, false), "a dead owner at once");
    let g = r.gpu.borrow();
    let b = r.d.blank.unwrap().resource_id;
    assert_eq!(g.scanouts[0], b, "the blank on scanout 0");
    assert!(g.shown(0).unwrap().iter().all(|&p| p == 0), "and it is zero");
    assert_eq!(g.scanouts[1], 0, "scanout 1 detached");
    assert!(!g.res.contains_key(&own), "the owner's resource unreferenced");
    assert!(g.res.values().all(|x| !x.px.contains(&SECRET)), "no host copy of the frame");
    assert!(primary_px(&r).iter().all(|&p| p == 0), "the primary zeroed");
    assert!(r.d.owner.quiet.get() && r.d.resources.lookup(own).is_none());
}

#[test]
fn the_primary_bring_up_showed_is_taken_off_too() {
    let r = rig();
    agree(&r, "bring-up");
    let p = r.d.primary.unwrap();
    assert_eq!(ask(&r, OWNER, OP_GET_PRIMARY_SURFACE, &[], 0).0, 0);
    fill(&r, SECRET);
    let all = words(&[p.resource_id, 0, 0, p.width, p.height]);
    assert_eq!(ask(&r, OWNER, OP_TRANSFER_TO_HOST, &with64(all.clone(), 24, 0), 0).0, 0);
    assert_eq!(ask(&r, OWNER, OP_FLUSH, &all, 0).0, 0);
    assert!(watch(&r.d, 100, false));
    clean(&r, 0, "never set by the owner");
}
