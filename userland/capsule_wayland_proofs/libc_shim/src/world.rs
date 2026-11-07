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

//! What the host kernel double remembers for the thread it runs on.

use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SurfaceDescriptor {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub byte_len: u64,
    pub base_va: u64,
    pub flags: u64,
    pub owner_pid: u32,
    pub writable: u32,
}

#[derive(Default)]
pub struct World {
    /// Every debug line the server wrote, in order.
    pub said: Vec<Vec<u8>>,
    /// Surfaces registered, by id less one; None once released.
    pub surfaces: Vec<Option<SurfaceDescriptor>>,
    /// Frames waiting in the server's inbox, with their sender.
    pub inbox: VecDeque<(u32, Vec<u8>)>,
    /// Whether debug lines are echoed to stderr as they are said.
    pub echo: bool,
    /// Each surface's release count, by id less one (MkSurfaceFenceWait).
    pub fences: Vec<u32>,
    /// The longest a fence wait was asked to sleep, in milliseconds.
    pub longest_wait_ms: u64,
    /// Pins made, as the kernel makes them; refused while false, as a
    /// kernel refuses one it cannot hold, so the copy path is what shows.
    pub allow_pins: bool,
    /// Pins made and given up so far.
    pub pinned: u32,
    pub unpinned: u32,
    /// The readers each pin asked for, in order, refused or not: the
    /// compositor and the scanout driver (0 for none).
    pub pin_readers: Vec<(u32, u32)>,
    /// A pin naming a scanout reader refused, as a kernel refuses a pid
    /// that serves no display driver's endpoint.
    pub refuse_scanout: bool,
    /// Pages mapped and not yet unmapped, as (start, length), and every
    /// unmap so far (MkMmap, MkMunmap).
    pub maps: Vec<(usize, usize)>,
    pub unmaps: Vec<(usize, usize)>,
    /// Unmaps that took pages a surface still registered names.
    pub unmapped_registered: u32,
}

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::default());
}

/// Run `f` over this thread's world.
pub fn with<R>(f: impl FnOnce(&mut World) -> R) -> R {
    WORLD.with(|w| f(&mut w.borrow_mut()))
}

pub fn say(line: &[u8]) {
    with(|w| {
        if w.echo {
            eprint!("{}", String::from_utf8_lossy(line));
        }
        w.said.push(line.to_vec());
    });
}

/// A handle as the kernel's look: the low word the same for every surface
/// (0xA_0000_0001, 0xB_0000_0001 on a booted image), the count above it.
pub fn register(desc: SurfaceDescriptor) -> i64 {
    with(|w| {
        w.surfaces.push(Some(desc));
        ((w.surfaces.len() as i64) << 32) | 1
    })
}

fn slot(handle: u64) -> usize {
    (handle >> 32).wrapping_sub(1) as usize
}

/// A shared handle is the surface's own id here: one compositor maps it.
pub fn share(sid: u64) -> i64 {
    with(|w| match w.surfaces.get(slot(sid)) {
        Some(Some(_)) => sid as i64,
        _ => -22,
    })
}

pub fn release(handle: u64) -> i64 {
    with(|w| match w.surfaces.get_mut(slot(handle)) {
        Some(slot @ Some(_)) => {
            *slot = None;
            0
        }
        _ => -22,
    })
}

/// The surface behind `handle`, while it is registered.
pub fn surface(handle: u64) -> Option<SurfaceDescriptor> {
    with(|w| w.surfaces.get(slot(handle)).copied().flatten())
}

pub fn post(sender: u32, frame: Vec<u8>) {
    with(|w| w.inbox.push_back((sender, frame)));
}

pub fn next_frame() -> Option<(u32, Vec<u8>)> {
    with(|w| w.inbox.pop_front())
}

/// The compositor released `handle`'s buffer: its count moves on.
pub fn fence_release(handle: u64) -> i64 {
    with(|w| {
        let i = slot(handle);
        if !matches!(w.surfaces.get(i), Some(Some(_))) {
            return -22;
        }
        if w.fences.len() <= i {
            w.fences.resize(i + 1, 0);
        }
        w.fences[i] = w.fences[i].wrapping_add(1);
        i64::from(w.fences[i])
    })
}

/// Surface `sid`'s release count, or EINVAL for one not registered. The
/// host never sleeps: a count that has not moved is answered as it is,
/// and the sleep asked for is noted (`longest_wait_ms`).
pub fn fence_wait(sid: u64, timeout_ms: u64) -> i64 {
    with(|w| {
        w.longest_wait_ms = w.longest_wait_ms.max(timeout_ms);
        let i = slot(sid);
        match w.surfaces.get(i) {
            Some(Some(_)) => i64::from(w.fences.get(i).copied().unwrap_or(0)),
            _ => -22,
        }
    })
}

/// MkPeerPin: the span registered as a surface, its handle the answer;
/// EPERM while the world allows no pins, or a scanout reader is named
/// while the world refuses one.
pub fn pin(desc: SurfaceDescriptor, readers: (u32, u32)) -> i64 {
    with(|w| w.pin_readers.push(readers));
    if !with(|w| w.allow_pins && !(w.refuse_scanout && readers.1 != 0)) {
        return -1;
    }
    let handle = register(desc);
    with(|w| w.pinned += 1);
    handle
}

/// MkPeerUnpin: the pin's surface released.
pub fn unpin(handle: u64) -> i64 {
    let rc = release(handle);
    if rc == 0 {
        with(|w| w.unpinned += 1);
    }
    rc
}
