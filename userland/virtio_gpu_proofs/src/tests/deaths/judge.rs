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
//! What a blank gone wrong at any command still owes, from the host's log
//! of the commands it carried out: a picture of the owner's stays on a
//! scanout only while the command that takes it off has not landed, and
//! the same for every copy of it the host holds.

use super::draw::{primary_px, SECRET};
use super::kit::agree;
use super::rig::Rig;
use crate::constants::{
    VG_CMD_RESOURCE_FLUSH, VG_CMD_RESOURCE_UNREF, VG_CMD_SET_SCANOUT, VG_CMD_TRANSFER_TO_HOST_2D,
};

/// Whether a command of `kind` on `id` (and scanout `s`) landed from `from`.
pub fn landed(r: &Rig, from: u32, kind: u32, id: u32, s: u32) -> bool {
    let g = r.gpu.borrow();
    let cmds = &g.log[from as usize..];
    cmds.iter().any(|c| c.landed && c.kind == kind && c.id == id && c.scanout == s)
}

/// `lived`: the driver ran to the end, so its table is still its word.
pub fn judge(r: &Rig, from: u32, own: u32, lived: bool, tag: &str) {
    let p = r.d.primary.unwrap().resource_id;
    let zeroed = landed(r, from, VG_CMD_TRANSFER_TO_HOST_2D, p, 0);
    let g = r.gpu.borrow();
    let secret = |px: Option<&[u32]>| px.is_some_and(|px| px.contains(&SECRET));
    for s in 0..2u32 {
        let off = landed(r, from, VG_CMD_SET_SCANOUT, 0, s);
        let cleared = s == 0 && g.scanouts[0] == p && zeroed;
        assert!(!secret(g.shown(s as usize)) || !(off || cleared), "{tag}: scanout {s}");
    }
    // Scanout 0 goes first: once the host took anything, it shows none.
    let took = g.log.get(from as usize).is_some_and(|c| c.landed);
    assert!(!took || !secret(g.shown(0)), "{tag}: scanout 0 off first");
    let host_p = g.res.get(&p).map(|x| &x.px[..]);
    assert!(!secret(host_p) || !zeroed, "{tag}: the host's primary");
    let unref = landed(r, from, VG_CMD_RESOURCE_UNREF, own, 0);
    let host_own = g.res.get(&own).map(|x| &x.px[..]);
    assert!(!secret(host_own) || !unref, "{tag}: the owner's resource");
    assert!(!lived || !unref || r.d.resources.lookup(own).is_none(), "{tag}: the table");
}

/// After a blank that landed whole: nothing of the owner's anywhere.
pub fn clean(r: &Rig, own: u32, tag: &str) {
    let g = r.gpu.borrow();
    let b = r.d.blank.unwrap().resource_id;
    assert_eq!((g.scanouts[0], g.scanouts[1]), (b, 0), "{tag}: blank and detached");
    assert!(g.shown(0).unwrap().iter().all(|&p| p == 0), "{tag}: the blank is zero");
    assert!(g.res.values().all(|x| !x.px.contains(&SECRET)), "{tag}: no host copy");
    assert!(!g.res.contains_key(&own) && r.d.resources.lookup(own).is_none(), "{tag}: own");
    assert!(primary_px(r).iter().all(|&p| p == 0), "{tag}: the primary zeroed");
    assert!(r.d.owner.quiet.get(), "{tag}: whole");
    drop(g);
    agree(r, tag);
    assert!(landed(r, 0, VG_CMD_RESOURCE_FLUSH, b, 0), "{tag}: the blank flushed");
}
