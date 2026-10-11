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
//! A pool is read where the client mapped its descriptor, and a client that
//! grows the file usually maps it again, often at another address. So the
//! descriptor is looked at again, and the pool moves with it; when the
//! client has closed it, the pool stays where it was. Either way the size is
//! what the client asked for, bounded by what is really mapped there, and
//! every commit checks its buffer against it (frame_len.rs).

use crate::linux::file::mapped_at;
use crate::linux::guest::Guest;

use super::args::Args;

pub fn resize(guest: &mut Guest, pool: u32, args: &mut Args<'_>) {
    let Some(asked) = args.u32() else { return };
    let Some(p) = guest.scene.pools.iter().find(|p| p.id == pool) else { return };
    let at = mapped_at(guest, u64::from(p.fd)).map_or(p.at, |(at, _)| at);
    let size = u64::from(asked).min(guest.mapped_from(at));
    if size == 0 {
        return;
    }
    if let Some(p) = guest.scene.pools.iter_mut().find(|p| p.id == pool) {
        (p.at, p.size) = (at, size);
    }
}
