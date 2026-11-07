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

//! The cursor plane (CURSOR on the display wire), where the device has a
//! cursor queue: one 64 x 64 resource the shapes are loaded into from the
//! output's cursor slot (copied into its DMA and sent with
//! TRANSFER_TO_HOST_2D on the control queue), and per scanout an
//! UPDATE_CURSOR when the shape or hot spot it shows changes, or the
//! cursor is shown or hidden, and a MOVE_CURSOR otherwise. Pointer motion
//! is one 56-byte command, with nothing recomposited or transferred. Any
//! refusal is the compositor's cue to draw the pointer itself.

use nonos_gfx_serve::MAX_OUTPUTS;
use nonos_gfx_wire::body::cursor::{MAX_SIDE, NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::errno::{EINVAL, ENOSYS, ENXIO, ETIMEDOUT};
use nonos_gfx_wire::{Cursor, Errno, Rect};

use super::describe;
use super::gpu::Gpu;
use super::plug::Plug;
use super::surface::Surface;
use crate::device::cmd::cursor::{movement, update, Place};
use crate::device::virtqueue::CursorQueue;

const SIDE: u32 = MAX_SIDE as u32;
/// The image a shape is: 64 x 64 pixels of four bytes, as the slot holds it.
pub const IMAGE: usize = (SIDE * SIDE * 4) as usize;

pub struct Plane {
    pub queue: CursorQueue,
    /// The resource the shapes are loaded into, made with the first.
    pub image: Option<Surface>,
    /// Per output, the hot spot of the shape its scanout shows, if any.
    pub told: [Option<(u16, u16)>; MAX_OUTPUTS],
}

impl Plane {
    pub fn new(queue: CursorQueue) -> Self {
        Self { queue, image: None, told: [None; MAX_OUTPUTS] }
    }
}

impl Gpu {
    /// `c` on output `o`'s scanout, its shape read from `slot` when new.
    pub fn show_cursor(&mut self, o: u16, c: &Cursor, slot: &[u8]) -> Result<(), Errno> {
        describe::live(self)?;
        let h = self.head(o).ok_or(ENXIO)?;
        let (scanout, plug) = (h.scanout, h.plug);
        let plane = self.cursor.as_mut().ok_or(ENOSYS)?;
        if plane.queue.is_out() {
            return Err(ENXIO);
        }
        let told = plane.told.get_mut(o as usize).ok_or(ENXIO)?;
        if plug != Plug::In {
            // A dark scanout shows nothing; the repaint on its return
            // sends the cursor again, and it is told the shape afresh.
            *told = None;
            return Ok(());
        }
        if c.flags & NEW_SHAPE != 0 {
            *told = None;
            self.load_shape(slot)?;
        }
        let at = Place {
            scanout,
            x: c.x.saturating_sub(i32::from(c.hot_x)),
            y: c.y.saturating_sub(i32::from(c.hot_y)),
        };
        self.place(o, at, c)
    }

    /// The UPDATE_CURSOR or MOVE_CURSOR `c` needs on output `o`, if any.
    fn place(&mut self, o: u16, at: Place, c: &Cursor) -> Result<(), Errno> {
        let plane = self.cursor.as_mut().ok_or(ENOSYS)?;
        let image = plane.image.as_ref().map(|s| s.id);
        let told = plane.told.get_mut(o as usize).ok_or(ENXIO)?;
        let hot = (c.hot_x, c.hot_y);
        let (request, now) = match (c.flags & VISIBLE != 0, image) {
            (false, _) if told.is_none() => return Ok(()),
            (false, _) => (update(at, 0, (0, 0)), None),
            (true, None) => return Err(EINVAL),
            (true, Some(_)) if *told == Some(hot) => (movement(at), *told),
            (true, Some(id)) => (update(at, id, (hot.0.into(), hot.1.into())), Some(hot)),
        };
        plane.queue.send(&request).map_err(|_| ETIMEDOUT)?;
        *told = now;
        Ok(())
    }

    /// The slot's image in the cursor resource, on the host's copy too.
    fn load_shape(&mut self, slot: &[u8]) -> Result<(), Errno> {
        let img = slot.get(..IMAGE).ok_or(EINVAL)?;
        let made = match self.cursor.as_mut().ok_or(ENOSYS)?.image.take() {
            Some(s) => s,
            None => self.make(SIDE, SIDE)?,
        };
        // SAFETY: the surface's DMA is mapped, at least its bytes() long,
        // for as long as the surface is this driver's; a 64 x 64 surface
        // is IMAGE bytes, and `img` is IMAGE bytes of the caller's slot.
        unsafe {
            core::ptr::copy_nonoverlapping(img.as_ptr(), made.map.user_va as *mut u8, IMAGE);
        }
        let whole = [Rect { x: 0, y: 0, width: SIDE, height: SIDE }];
        let sent = self.send(&made, &whole);
        let plane = self.cursor.as_mut().ok_or(ENOSYS)?;
        plane.image = Some(made);
        sent
    }
}
