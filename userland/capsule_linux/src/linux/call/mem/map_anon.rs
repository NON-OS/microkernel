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

//! Mappings with no file behind them.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::map_req::MapReq;
use super::prot::{PROT_EXEC, PROT_WRITE};
use nonos_linux_policy::mask::JIT;

const MAP_SHARED: u64 = 0x01;

pub fn anonymous(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    if req.prot & PROT_EXEC != 0 && !crate::linux::app::granted(guest, JIT) {
        return crate::linux::app::refuse("executable anonymous memory", "JIT", errno::EPERM);
    }
    if !req.make_room(guest, at, span) {
        return super::map_refused::refused("no room", span);
    }
    /*
     * A PROT_NONE anonymous mapping is a reservation: the runtime that makes it
     * (Go's, for one) commits a fraction of it later with a fixed RW mapping.
     * Backing the whole span here would spend real frames on address space no
     * one may touch, so reserve it; a commit maps the part that is opened.
     */
    let backed = if req.prot == 0 {
        guest.reserve(at, span)
    } else {
        /* A fixed anonymous span reads as zeros; drop frames map would keep. */
        if req.fixed().is_some() {
            guest.drop_frames(at, span);
        }
        guest.map(at, span, req.prot & PROT_WRITE != 0, req.prot & PROT_EXEC != 0)
    };
    if backed < 0 {
        return super::map_refused::refused("no memory to back it", span);
    }
    if req.flags & MAP_SHARED != 0 {
        /* Shared pages keep their bytes after MADV_DONTNEED on Linux. */
        guest.mark_kept(at, span);
    }
    errno::ok(at)
}
