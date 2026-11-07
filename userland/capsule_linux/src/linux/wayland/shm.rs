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

//! `wl_shm`: the pool a client draws into, and the buffers inside it.

use crate::linux::file::memfd_id;
use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::ops::{ev, FORMAT_ARGB8888, FORMAT_XRGB8888};
use super::out::Event;
use super::state::Pool;

/// A client reads the format list before it asks for anything, and one it was
/// not offered is a protocol error on its side, so only the two that are
/// actually blitted are advertised.
pub fn formats(guest: &mut Guest, id: u32) {
    for format in [FORMAT_ARGB8888, FORMAT_XRGB8888] {
        Event::new(id, ev::SHM_FORMAT).u32(format).send(&mut guest.display.to_client);
    }
}

/// The descriptor came in the control data of the sendmsg that carried this
/// request, taken from the queue in order; the body is the new id and size.
pub fn create_pool(guest: &mut Guest, args: &mut Args<'_>) {
    // Taken first, whatever the body says, so the next request that passes
    // one is given its own.
    let fd = take_fd(guest);
    let (Some(id), Some(size)) = (args.u32(), args.u32()) else {
        fd.into_iter().for_each(|fd| crate::linux::unix::passed::drop(guest, fd));
        return;
    };
    let Some(fd) = fd else {
        return refused(b"[WAYLAND] shm pool with no descriptor passed\n");
    };
    // Kept by the pool while it lives (gone_pools), else closed now.
    if !made(guest, id, size, fd) {
        crate::linux::unix::passed::drop(guest, fd);
    }
}

/// A pool over memfd `fd`. Where its pixels are is not settled here: a
/// client may have let its mapping go before this request is read (Qt
/// replaces a buffer as a window is resized, unmapping the old one at once),
/// and one that names the pool's id afterwards was killed for a pool it had
/// made. So the pool is made over the memfd at the size the client gave, and
/// every read of a buffer in it finds the memfd mapped now, whole under the
/// buffer, or shows nothing (present/source.rs).
fn made(guest: &mut Guest, id: u32, size: u32, fd: u32) -> bool {
    // A memfd only: its bytes are the guest's mapping of it. Any other file
    // shared this way has no pages here to read.
    let Some(memfd) = memfd_id(guest, fd as u64) else {
        refused(b"[WAYLAND] shm pool over a descriptor that is no memfd\n");
        return false;
    };
    if size == 0 {
        refused(b"[WAYLAND] shm pool of no size\n");
        return false;
    }
    if !guest.objects.put(id, Object::ShmPool) {
        refused(b"[WAYLAND] shm pool refused: the object table is full\n");
        return false;
    }
    if crate::linux::guest::memfd_spans::reach(&guest.memfd_spans, memfd).is_none() {
        refused(b"[WAYLAND] shm pool over a memfd not mapped now: not shown until it is\n");
    }
    guest.scene.pools.push(Pool { id, size: u64::from(size), fd, memfd });
    true
}

/// The pools that go, as `keep` says: their kept descriptors closed.
pub fn drop_pools(guest: &mut Guest, keep: impl Fn(&Guest, &Pool) -> bool) {
    let (stay, gone): (alloc::vec::Vec<Pool>, alloc::vec::Vec<Pool>) =
        core::mem::take(&mut guest.scene.pools).into_iter().partition(|p| keep(guest, p));
    guest.scene.pools = stay;
    gone.into_iter().for_each(|p| crate::linux::unix::passed::drop(guest, p.fd));
}

/// The first of the ids a destroyed pool keeps while its buffers live. The
/// client is told its id is free and may give it to a new pool at once, as
/// libwayland does when a pool is made again for a new size; the old pool
/// moves to an id of the server's range, which a client never names, so its
/// buffers go on reading their own memory.
const KEPT_FROM: u32 = 0xFF80_0000;

/// Pool `id` destroyed: if a buffer is still made from it, it and its
/// buffers take an id of the server's own.
pub fn kept(scene: &mut super::scene::Scene, id: u32) {
    if !scene.buffers.iter().any(|b| b.pool == id) {
        return;
    }
    let next = scene.pools.iter().map(|p| p.id).filter(|&p| p >= KEPT_FROM).max();
    let Some(key) = next.map_or(Some(KEPT_FROM), |m| m.checked_add(1)) else { return };
    scene.pools.iter_mut().filter(|p| p.id == id).for_each(|p| p.id = key);
    scene.buffers.iter_mut().filter(|b| b.pool == id).for_each(|b| b.pool = key);
}

/// A pool not made says why: else the client fails later, on no object.
fn refused(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

pub(super) fn take_fd(guest: &mut Guest) -> Option<u32> {
    match guest.display.fds.is_empty() {
        true => None,
        false => Some(guest.display.fds.remove(0)),
    }
}
