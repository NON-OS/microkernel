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
use crate::linux::guest::{maps_full, Guest};

use super::map_args::map_args;
use super::map_place::place;
use super::map_req::MapReq;

pub fn mmap(guest: &mut Guest, req: MapReq) -> u64 {
    /*
     * Every refusal the arguments alone decide, in Linux's order, with this
     * personality's own two: no page both writable and executable, and no
     * exact address below mmap_min_addr, since landing elsewhere would hand
     * back memory the guest did not ask for.
     */
    if let Err(e) = map_args(req.addr, req.len, req.prot, req.flags, req.off) {
        return errno::fail(e);
    }
    /* Linux refuses a mapping past vm.max_map_count with ENOMEM. */
    if maps_full(guest.regions.len(), guest.regions.len() + 1) {
        return errno::fail(errno::ENOMEM);
    }
    let spot = match place(guest, &req) {
        Ok(spot) => spot,
        Err(e) => return errno::fail(e),
    };
    let out = super::map_kind::map_at(guest, &req, spot.at, spot.span);
    if spot.from_cursor && (out as i64) >= 0 {
        guest.mmap_next = spot.at + spot.span;
    }
    out
}
