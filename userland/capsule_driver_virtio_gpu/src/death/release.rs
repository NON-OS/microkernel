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
//! An owner's own resources given up (G-73): each one's backing detached
//! and the resource unreferenced, so the host keeps no copy of what was
//! drawn there. One the device did not let go of stays in the table owned
//! by no one: never handed out, its id never reused, and tried again with
//! the next blank.

use crate::device::cmd;
use crate::driver::Driver;

pub fn release(d: &Driver, pid: u32) -> bool {
    let mine = |owner: u32| owner == 0 || (pid != 0 && owner == pid);
    let (ids, n) = d.resources.ids_where(|r| mine(r.owner_pid) && !d.ours(r.resource_id));
    let mut whole = true;
    for &id in &ids[..n] {
        whole &= let_go(d, id);
    }
    whole
}

fn let_go(d: &Driver, id: u32) -> bool {
    let q = &d.control_queue;
    let attached = d.resources.lookup(id).is_some_and(|r| r.backing_addr != 0);
    if attached && cmd::detach_backing(q, d.fences.issue(), id).is_err() {
        d.resources.update(id, |r| r.owner_pid = 0);
        return false;
    }
    d.resources.update(id, |r| (r.backing_addr, r.backing_len) = (0, 0));
    if cmd::resource_unref(q, d.fences.issue(), id).is_err() {
        d.resources.update(id, |r| r.owner_pid = 0);
        return false;
    }
    d.resources.remove(id);
    true
}
