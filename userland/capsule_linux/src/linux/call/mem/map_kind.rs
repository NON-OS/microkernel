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

//! Which kind of mapping an mmap makes once it has a place.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::map_anon::anonymous;
use super::map_file::file;
use super::map_memfd::memfd;
use super::map_req::MapReq;

const MAP_SHARED: u64 = 0x01;
const MAP_ANONYMOUS: u64 = 0x20;

pub(super) fn map_at(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    if req.flags & MAP_ANONYMOUS != 0 {
        return anonymous(guest, req, at, span);
    }
    if crate::linux::file::is_memfd(guest, req.fd) {
        return memfd(guest, req, at, span);
    }
    // shm_open's object, mapped shared, is a memfd from here (shm_object.rs).
    if req.flags & MAP_SHARED != 0 && crate::linux::file::is_shm_object(guest, req.fd, req.off) {
        if let Err(e) = crate::linux::file::into_memfd(guest, req.fd) {
            return errno::fail(e);
        }
        return memfd(guest, req, at, span);
    }
    if req.flags & MAP_SHARED != 0 {
        /*
         * Sharing a file between processes needs frames that two address
         * spaces both point at, which no peer call offers: the family
         * serves it instead, each map with its own pages (map_shared.rs).
         */
        return super::map_shared::shared(guest, req, at, span).map_or_else(errno::fail, errno::ok);
    }
    file(guest, req, at, span)
}
