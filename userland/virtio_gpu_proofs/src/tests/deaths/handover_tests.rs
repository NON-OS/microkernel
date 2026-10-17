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
//! A restarted compositor takes the primary (only the compositor may send
//! here, so a new pid asking is one): after the dead one's blank, or before
//! the watch got to it, in which case the blank comes first. Either way the
//! new owner reads a zeroed primary, none of the old owner's resources is
//! left, and the old pid's requests are refused.

use super::ask::{ask, words, OWNER};
use super::draw::{draw, fill, present, primary_px, SECRET};
use super::rig::{rig, Rig};
use crate::death::watch::watch;
use crate::protocol::{E_BUSY, OP_FLUSH, OP_GET_PRIMARY_SURFACE};

pub const NEXT: u32 = 9;

pub fn handed(r: &Rig, own: u32, tag: &str) {
    let g = r.gpu.borrow();
    assert!(g.res.values().all(|x| !x.px.contains(&SECRET)), "{tag}: no host copy");
    assert!(g.scanouts[..2].iter().all(|&s| s != own), "{tag}: none shows its resource");
    assert!(r.d.resources.lookup(own).is_none(), "{tag}: its resource gone");
    assert!(primary_px(r).iter().all(|&p| p == 0), "{tag}: the primary zeroed");
    assert_eq!(r.d.owner.pid.get(), NEXT, "{tag}: the new owner");
}

fn take(r: &Rig, own: u32, tag: &str) {
    assert_eq!(ask(r, NEXT, OP_GET_PRIMARY_SURFACE, &[], 500).0, 0, "{tag}");
    handed(r, own, tag);
    let p = r.d.primary.unwrap();
    let all = words(&[p.resource_id, 0, 0, p.width, p.height]);
    assert_eq!(ask(r, OWNER, OP_FLUSH, &all, 600).0, E_BUSY, "{tag}: the old pid refused");
    fill(r, 0x77);
    present(r, NEXT, 600);
    assert!(r.gpu.borrow().shown(0).unwrap().iter().all(|&p| p == 0x77), "{tag}: shown");
}

#[test]
fn a_restarted_compositor_takes_the_primary_after_the_blank() {
    let r = rig();
    let own = draw(&r);
    assert!(watch(&r.d, 3_000, true));
    // Slow, not dead: it drew into the primary again after the blank.
    fill(&r, SECRET);
    take(&r, own, "after");
}

#[test]
fn a_restarted_compositor_before_the_watch_has_the_blank_run_first() {
    let r = rig();
    let own = draw(&r);
    take(&r, own, "before");
}
