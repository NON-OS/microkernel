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

//! `wl_shm_pool.resize`: the client grew the file behind a pool.
//!
//! A pool is read wherever its memfd is mapped when a buffer of it is read
//! (present/source.rs), so a client that maps the grown file again, often at
//! another address, is followed there. Only the size moves here: what the
//! client asked for, which every read bounds again by what it has mapped.

use crate::linux::guest::Guest;

use super::args::Args;

pub fn resize(guest: &mut Guest, pool: u32, args: &mut Args<'_>) {
    let Some(asked) = args.u32() else { return };
    if asked == 0 {
        return;
    }
    let Some(p) = guest.scene.pools.iter().find(|p| p.id == pool) else { return };
    let (fd, memfd) = (p.fd, p.memfd);
    // The descriptor the pool keeps still names its memfd; else it is left
    // as it was.
    if crate::linux::file::memfd_id(guest, u64::from(fd)) != Some(memfd) {
        return;
    }
    if let Some(p) = guest.scene.pools.iter_mut().find(|p| p.id == pool) {
        p.size = u64::from(asked);
    }
}
