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

//! When a client's buffer is scanned out with no composite (a project decision
//! of 2026-11-02, IOMMU-confined only), pure, so the host proofs hold it.
//! The topmost layer must cover the whole screen exactly, be a swapchain
//! buffer its client presented as opaque, at the screen's own size, on a
//! screen shown one to one with no look, lens or second output, the pointer
//! on the device's own plane, and a driver that keeps the band on a plane
//! of its own and has not refused. Anything else composes: a popup, an
//! overlay, a prompt in the trusted band, a window not filling the screen,
//! each is a layer above it or a layer that does not cover it. A layer
//! that draws a pinned client buffer over its own surface is scanned out
//! from that buffer only when it is put down opaque from the layer's corner
//! and fills the layer (seen).

/// What the screen and the driver allow this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Facts {
    /// The wire driver serves SCANOUT with the band on its own plane, and
    /// has not refused it for good.
    pub able: bool,
    /// The screen's size, which the canvas is.
    pub screen: (u32, u32),
    /// Shown one to one, no look, lens, night light or second output.
    pub plain: bool,
    /// The pointer is on the device's cursor plane.
    pub cursor_plane: bool,
}

/// The topmost layer of the scene and the buffer it shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Top {
    pub handle: u64,
    pub at: (u32, u32),
    pub size: (u32, u32),
    /// The buffer's own size and row stride in bytes, as attached.
    pub buffer: (u32, u32),
    pub stride: u32,
    /// Its last present said every pixel is opaque.
    pub opaque: bool,
    /// The driver refused this buffer (it may not read it).
    pub refused: bool,
}

/// A surface a layer draws over its own (OP_LAYER_CONTENT): a Linux
/// window's client buffer pinned in place.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Over {
    pub handle: u64,
    /// Its corner on the layer.
    pub at: (u32, u32),
    pub buffer: (u32, u32),
    pub stride: u32,
    /// Drawn opaque, its alpha byte not read: an XRGB buffer.
    pub opaque: bool,
    pub refused: bool,
    /// The layer draws all of the buffer, not a window's part of one that
    /// also holds its shadow, which the plane would show.
    pub whole: bool,
}

/// The one buffer a layer is seen as: its own surface when it draws no
/// content, the content when that is put down opaque from the layer's
/// corner, and so hides every pixel of the layer's own surface once pick
/// finds it the layer's size. None for a content drawn anywhere else,
/// blended, or only in part, which is two pictures or a part and composed.
pub fn seen(own: Top, over: Option<Over>) -> Option<Top> {
    let Some(c) = over else { return Some(own) };
    if c.handle == 0 || c.at != (0, 0) || !c.opaque || !c.whole {
        return None;
    }
    let (handle, buffer, stride, refused) = (c.handle, c.buffer, c.stride, c.refused);
    Some(Top { handle, buffer, stride, opaque: true, refused, ..own })
}

/// The buffer to scan out, or None to compose.
pub fn pick(f: &Facts, top: Option<&Top>) -> Option<u64> {
    let t = top?;
    let covers = t.at == (0, 0) && t.size == f.screen && t.buffer == f.screen;
    let rows = (t.stride as u64) >= (t.buffer.0 as u64) * 4;
    let ok = f.able && f.plain && f.cursor_plane && covers && rows && t.opaque && !t.refused;
    ok.then_some(t.handle)
}

/// What a driver's refusal of SCANOUT means for the next frames.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// Never again from this driver: no IOMMU confines the device, or the
    /// op is not served.
    Driver,
    /// Not this buffer: its owner did not let the driver read it, or it
    /// does not fit the plane.
    Buffer,
    /// Not now: a stale primary, a flip that timed out. Asked again later.
    Frame,
}

pub fn refusal(errno: i32) -> Refusal {
    use nonos_gfx_wire::errno::{EINVAL, ENODEV, ENOSYS, EPERM};
    match errno {
        ENODEV | ENOSYS => Refusal::Driver,
        EPERM | EINVAL => Refusal::Buffer,
        _ => Refusal::Frame,
    }
}

/// What a frame does: scan `want` out, or compose; and whether the whole
/// screen must be composed because the plane showed a client's buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Way {
    /// Send SCANOUT for this buffer (a new one, or the band changed).
    Scan(u64),
    /// The plane already shows it and nothing changed: nothing to send.
    Idle,
    /// Compose the damage; `whole` when the plane must come back to the
    /// primary, which then needs every pixel.
    Compose { whole: bool },
}

/// `plane`: the client buffer the plane shows now, by the driver's last
/// answer. `damaged`: this frame has damage (a present, the band).
pub fn way(want: Option<u64>, plane: Option<u64>, damaged: bool) -> Way {
    match (want, plane) {
        (Some(w), Some(p)) if w == p && !damaged => Way::Idle,
        (Some(w), _) => Way::Scan(w),
        (None, p) => Way::Compose { whole: p.is_some() },
    }
}
