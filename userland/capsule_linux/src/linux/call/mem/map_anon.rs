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

const MAP_SHARED: u64 = 0x01;

/// A memfd has nothing to read in: it is pages, and the client is about to
/// draw into them.
pub fn memfd(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    let out = anonymous(guest, req, at, span);
    if (out as i64) < 0 {
        return out;
    }
    guest.mark_kept(at, span);
    crate::linux::file::set_mapped(guest, req.fd, at);
    /*
     * A descriptor this capsule staged content on, the keymap being the one
     * that matters, has to arrive with those bytes already in it: the client
     * maps it and reads it without ever issuing a read.
     */
    if let Some(bytes) = crate::linux::file::staged(guest, req.fd) {
        if guest.write(at, &bytes) < bytes.len() as i64 {
            return errno::fail(errno::EFAULT);
        }
    }
    out
}

pub fn anonymous(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
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
