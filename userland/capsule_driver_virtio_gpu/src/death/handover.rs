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
//! A new owner for the primary: a restarted compositor asks for it.
//! What the last owner left is blanked first, if its death or silence has
//! not had that done whole already, and the primary is zeroed again before
//! it is handed on, so the new owner never reads a frame of the last.

use super::blank::{blank, zero_primary};
use crate::driver::Driver;

pub fn handover(d: &Driver, pid: u32, now: u64) {
    let last = d.owner.pid.get();
    if last == pid {
        return;
    }
    if last != 0 && !d.owner.quiet.get() {
        let _ = blank(d);
    }
    let _ = zero_primary(d);
    d.owner.adopt(pid, now);
    if let Some(p) = d.primary {
        d.resources.update(p.resource_id, |r| r.owner_pid = pid);
    }
}
