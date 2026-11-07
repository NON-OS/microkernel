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

//! The guest's buffers pinned so far, by wl_buffer, each over the span it
//! was pinned at. A buffer shown again at the same span is shown from the
//! same pin; one that moved, or was destroyed, gives its pin up.

use nonos_app_skeleton::clients::compositor::scanout_peer;
use nonos_libc::peer::{mk_peer_pin, mk_peer_unpin};
use nonos_libc::SurfaceDescriptor;

use super::super::super::reshape::Shape;
use super::super::super::scene::Scene;
use super::super::super::scene_window::Window;
use super::super::owed::Owed;

/// Pins one guest keeps: a client draws in two or three buffers, and the
/// kernel lets a guest hold 16 (pin_rule.rs PINS_PER_GUEST).
pub const MAX_PINS: usize = 8;

/// SURFACE_FORMAT_ARGB8888 in the surface registry (surface.rs).
const FORMAT_ARGB8888: u32 = 1;

/// The kernel's EPERM: among others, a scanout reader it will not name.
const EPERM: i64 = -1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pin {
    pub buffer: u32,
    /// The guest address and length of its rows, and their shape.
    pub at: u64,
    pub len: u64,
    pub shape: Shape,
    pub handle: u64,
    /// When it was last shown, for the one given up to make room.
    pub used: u64,
}

/// The pin over `buffer`'s rows at `span` in `shape`, made now when none
/// is, shared read-only with `reader` and with the display driver the
/// compositor on `port` names for scanout, so a full-screen window is
/// scanned out from it. None when the kernel refuses it, or every pin is
/// in use.
pub fn pin(
    scene: &mut Scene,
    (pid, reader, port): (u32, u32, u32),
    buffer: u32,
    (at, len): (u64, u64),
    shape: Shape,
    now: u64,
) -> Option<u64> {
    if let Some(p) = scene.pins.iter_mut().find(|p| p.buffer == buffer) {
        if (p.at, p.len, p.shape) == (at, len, shape) {
            p.used = now;
            return Some(p.handle);
        }
    }
    unpin(scene, pid, buffer);
    if scene.pins.len() >= MAX_PINS && !make_room(scene, pid) {
        return None;
    }
    let (width, height, stride) = shape;
    let format = FORMAT_ARGB8888;
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format,
        byte_len: len,
        base_va: at,
        flags: 0,
        owner_pid: 0,
        writable: 0,
    };
    let handle = pin_for(pid, &desc, reader, scanout_driver(port))?;
    scene.pins.push(Pin { buffer, at, len, shape, handle, used: now });
    Some(handle)
}

/// Pinned for the compositor and `scanout`; for the compositor alone when
/// the kernel refuses to name `scanout` (a driver restarted since the
/// compositor named it), so the window is still shown from its pin and
/// only composed.
fn pin_for(pid: u32, desc: &SurfaceDescriptor, reader: u32, scanout: u32) -> Option<u64> {
    let handle = |rc: i64| u64::try_from(rc).ok().filter(|&h| h != 0);
    match mk_peer_pin(pid, desc, reader, scanout) {
        EPERM if scanout != 0 => handle(mk_peer_pin(pid, desc, reader, 0)),
        rc => handle(rc),
    }
}

/// The display driver the compositor lets read a client buffer for
/// scanout, 0 when it names none. Asked under a request id of its own, as
/// window.rs asks it: the scene's serials are the configures'.
fn scanout_driver(port: u32) -> u32 {
    scanout_peer(port, 1).unwrap_or(0)
}

/// `buffer`'s pin given up, when it has one.
pub fn unpin(scene: &mut Scene, pid: u32, buffer: u32) {
    if let Some(i) = scene.pins.iter().position(|p| p.buffer == buffer) {
        let p = scene.pins.swap_remove(i);
        let _ = mk_peer_unpin(pid, p.handle);
    }
}

/// The guest unmapped or reprotected `[at, at + len)`: the kernel unpinned
/// what met it (peer_unmap.rs, peer_protect.rs), so no pin of it is shown
/// again.
pub fn unmapped(scene: &mut Scene, at: u64, len: u64) {
    let end = at.saturating_add(len);
    scene.pins.retain(|p| p.at.saturating_add(p.len) <= at || p.at >= end);
}

/// Whether `buffer` is what a window shows now, or waits to be given back.
pub fn in_use(scene: &Scene, buffer: u32) -> bool {
    let windows = core::iter::once(&scene.win).chain(scene.behind.iter());
    let shown = windows.filter_map(|w| w.content).any(|h| h.buffer == buffer);
    shown || scene.releasing.iter().any(|h| h.buffer == buffer)
}

/// Whether the client of `win`, which shows `buffer`, has the other buffer
/// it drew in before it (scene_window.rs turn) free: of `buffer`'s shape,
/// shown by no window, waiting to be given back by none and owed by none.
/// That buffer is its own again, to draw its next frame in, whether it was
/// last shown from a pin or by the copy.
pub fn spare(scene: &Scene, win: &Window, buffer: u32) -> bool {
    let Some(other) = win.turn.filter(|&b| b != buffer) else { return false };
    let shape = |id: u32| {
        let b = scene.buffers.iter().find(|b| b.id == id)?;
        Some((b.width, b.height, b.stride))
    };
    let same = shape(buffer).is_some_and(|s| shape(other) == Some(s));
    same && !in_use(scene, other) && !owed(scene, other)
}

/// Whether a window still owes a draw from the client's `buffer`, which it
/// keeps until then (present/owed.rs).
fn owed(scene: &Scene, buffer: u32) -> bool {
    let windows = core::iter::once(&scene.win).chain(scene.behind.iter());
    windows.filter_map(|w| w.owing).any(|o| o.what == Owed::Guest { buffer })
}

/// The pin shown longest ago and in no use given up; false when every pin
/// is in use.
fn make_room(scene: &mut Scene, pid: u32) -> bool {
    let free = scene.pins.iter().filter(|p| !in_use(scene, p.buffer));
    let Some(oldest) = free.min_by_key(|p| p.used).map(|p| p.buffer) else { return false };
    unpin(scene, pid, oldest);
    true
}
