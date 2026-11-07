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

//! A desktop on the host composed by the compositor's own code: the scene
//! table, the damage accumulator, the attach cache, `compose` and the batch
//! drain the wire path uses. Only the display is modelled: a flip copies the
//! composed rectangles into a second buffer, as the GOP present and a
//! virtio-gpu transfer each copy them.

use crate::attach::AttachCache;
use crate::damage::{DamageAccumulator, Rect};
use crate::frame_pacer::compose::{compose, Scene};
use crate::frame_pacer::drain_batch::drain_batched;
use crate::frame_pacer::trace::record::FrameRecord;
use crate::scene::{Layer, SceneTable};
use crate::scene_submit::submit_layer;
use crate::sw_blitter::Surface;

use super::clock::Clock;
use super::kernel::MemKernel;

pub struct Desk {
    pub width: u32,
    pub height: u32,
    pub canvas: Vec<u32>,
    pub scanout: Vec<u32>,
    pub scene: SceneTable,
    pub attach: AttachCache,
    pub kernel: MemKernel,
    pub damage: DamageAccumulator,
    pub cursor: Option<(u32, u32)>,
}

impl Desk {
    pub fn new(width: u32, height: u32) -> Desk {
        let n = (width * height) as usize;
        Desk {
            width,
            height,
            canvas: vec![0; n],
            scanout: vec![0; n],
            scene: SceneTable::new(),
            attach: AttachCache::new(),
            kernel: MemKernel::default(),
            damage: DamageAccumulator::new(),
            cursor: None,
        }
    }

    /// Submit a layer as the scene handler does, damaging where it is and
    /// where it was.
    pub fn submit(&mut self, l: Layer) {
        if let Ok(rects) = submit_layer(&mut self.scene, l) {
            rects.into_iter().flatten().for_each(|r| self.damage.accumulate(r));
        }
    }

    fn surfaces(&mut self) -> (Surface, *mut u32) {
        let dst = Surface {
            base_va: self.canvas.as_mut_ptr() as u64,
            stride: self.width * 4,
            width: self.width,
            height: self.height,
            byte_len: self.canvas.len() as u64 * 4,
        };
        (dst, self.scanout.as_mut_ptr())
    }

    /// One frame: drain the damage, compose each rectangle, flip each batch.
    pub fn frame(&mut self, clock: &Clock) -> FrameRecord {
        let mut rec = FrameRecord { start: clock.now(), ..FrameRecord::default() };
        let (dst, out) = self.surfaces();
        let width = self.width as usize;
        let cursor = self.cursor;
        let prepare = |d: &mut (&mut Desk, &mut FrameRecord), r: Rect| {
            let since = clock.now();
            let (desk, rec) = d;
            let s = Scene {
                scene: &mut desk.scene,
                attach: &mut desk.attach,
                kernel: &mut desk.kernel,
                damage: &mut desk.damage,
                ceiling: u32::MAX,
            };
            compose(dst, r, s, cursor);
            rec.composed(clock.now() - since, r.width, r.height);
            Some(r)
        };
        let send = |d: &mut (&mut Desk, &mut FrameRecord), rects: &[Rect]| -> Result<(), ()> {
            let since = clock.now();
            for r in rects {
                for y in r.y..r.y + r.height {
                    let at = y as usize * width + r.x as usize;
                    // SAFETY: the rectangle was clipped to the canvas, and
                    // the scanout buffer is the canvas's size.
                    unsafe {
                        let src = (dst.base_va as *const u32).add(at);
                        core::ptr::copy_nonoverlapping(src, out.add(at), r.width as usize);
                    }
                }
            }
            d.1.flipped(clock.now() - since);
            Ok(())
        };
        let mut both = (&mut *self, &mut rec);
        let _ = drain_batched(&mut both, |d| &mut d.0.damage, prepare, send);
        rec.end = clock.now();
        rec
    }
}
