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

//! A stored file mapped MAP_SHARED. Its pages are laid down as a private
//! map's are, from the file and zero past its end, and the family's table
//! then keeps them and the file the same (file/held/shared_maps).

use crate::linux::abi::errno;
use crate::linux::file::shared_maps;
use crate::linux::guest::Guest;

use super::map_args::MAP_FIXED;
use super::map_fill::fill_read;
use super::map_req::MapReq;
use super::prot_span::protect_span;

pub(super) fn shared(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> Result<u64, i64> {
    /*
     * Asked before a page is allocated: the descriptor, the protection, and
     * whether the map may stand beside the family's others.
     */
    let entry = shared_maps::asked(guest, req.fd, at, span, req.off, req.prot)?;
    shared_maps::admit(&entry, req.flags & MAP_FIXED != 0)?;
    if !req.make_room(guest, at, span) || guest.map(at, span, true, false) < 0 {
        return Err(errno::ENOMEM);
    }
    /* Linux reloads a file's bytes after MADV_DONTNEED; this capsule cannot. */
    guest.mark_kept(at, span);
    /* Whole pages, as a write of the file reaches them (shared_maps/echo.rs). */
    let whole = MapReq { len: span, ..*req };
    if fill_read(guest, &whole, at) < 0 || protect_span(guest, at, span, req.prot) < 0 {
        let _ = guest.unmap(at, span);
        return Err(errno::EACCES);
    }
    /* Nothing proved it, and nothing may run it (shared_maps/asks.rs). */
    guest.mark_unproven(at, span);
    shared_maps::record(entry);
    Ok(at)
}
