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

//! `mmap`: anonymous pages, or a private mapping of a file.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, PAGE};

use super::map_anon::{anonymous, memfd};
use super::map_file::file;
use super::map_place::place;
use super::map_req::MapReq;
use super::prot::wx_refused;

const MAP_SHARED: u64 = 0x01;
const MAP_ANONYMOUS: u64 = 0x20;
const MAP_FIXED: u64 = 0x10;
const MAP_FIXED_NOREPLACE: u64 = 0x10_0000;

pub fn mmap(guest: &mut Guest, req: MapReq) -> u64 {
    // Linux takes a file offset on a page boundary, and an exact address too;
    // only a hint is rounded.
    let exact = req.flags & (MAP_FIXED | MAP_FIXED_NOREPLACE) != 0;
    if req.len == 0 || req.off % PAGE != 0 || (exact && req.addr % PAGE != 0) {
        return errno::fail(errno::EINVAL);
    }
    if wx_refused(req.prot) {
        return errno::fail(errno::EPERM);
    }
    // MAP_FIXED and MAP_FIXED_NOREPLACE are the exact address or failure.
    // Page zero is never in the plan, and landing elsewhere would hand back
    // memory the guest did not ask for, so it is refused, as Linux refuses it
    // below mmap_min_addr.
    if exact && req.addr == 0 {
        return errno::fail(errno::EPERM);
    }
    let spot = match place(guest, &req) {
        Ok(spot) => spot,
        Err(e) => return errno::fail(e),
    };
    let out = map_at(guest, &req, spot.at, spot.span);
    if spot.from_cursor && (out as i64) >= 0 {
        guest.mmap_next = spot.at + spot.span;
    }
    out
}

fn map_at(guest: &mut Guest, req: &MapReq, at: u64, span: u64) -> u64 {
    if req.flags & MAP_ANONYMOUS != 0 {
        return anonymous(guest, req, at, span);
    }
    if crate::linux::file::is_memfd(guest, req.fd) {
        return memfd(guest, req, at, span);
    }
    if req.flags & MAP_SHARED != 0 {
        /*
         * Sharing a file between processes needs frames that two address
         * spaces both point at, which no peer call offers.
         */
        return errno::fail(errno::ENOSYS);
    }
    file(guest, req, at, span)
}
