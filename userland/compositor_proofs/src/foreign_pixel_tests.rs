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

//! Real pixels, composed by the compositor's own `compose`, from surfaces in
//! memory the way the kernel hands them over. A surface whose owner exits
//! goes back to the kernel, which gives its frames to other processes: an
//! image viewer's decoded picture, a wallpaper. Here that memory is filled
//! with FOREIGN the moment the owner exits, and no composed pixel may ever
//! be FOREIGN again. The pointer moves across the screen in a staircase, as
//! on ek's screen, so every rectangle it damages is composed afresh.

use std::collections::HashMap;

use crate::attach::{AttachCache, SurfaceKernel};
use crate::damage::{DamageAccumulator, Rect};
use crate::frame_pacer::compose::{compose, Scene};
use crate::scene::{Layer, SceneTable};
use crate::scene_submit::submit_layer;
use crate::sw_blitter::Surface;

const W: u32 = 96;
const H: u32 = 64;
const CURSOR_SIDE: u32 = 32;
const WALL: u32 = 0xFF20_6070;
const WINDOW: u32 = 0xFFE0_E0E0;
const OTHER: u32 = 0xFF30_A040;
/// What the kernel's next owner of a freed frame put there.
const FOREIGN: u32 = 0xFFFF_00FF;

/// The kernel's side: each surface's memory, and which handles it still
/// knows for the compositor.
#[derive(Default)]
struct Kernel {
    memory: HashMap<u64, (Vec<u32>, u32, u32, u32)>,
    live: Vec<u64>,
}

impl Kernel {
    /// A surface of `w` by `h` pixels, `stride_px` pixels a row, painted
    /// `colour`; the slack past each row and past the end holds FOREIGN.
    fn register(&mut self, handle: u64, w: u32, h: u32, stride_px: u32, colour: u32) {
        let mut px = vec![FOREIGN; (stride_px * h + 64) as usize];
        for y in 0..h {
            for x in 0..w {
                px[(y * stride_px + x) as usize] = colour;
            }
        }
        self.memory.insert(handle, (px, w, h, stride_px));
        self.live.push(handle);
    }

    /// The owner exits: the kernel forgets the surface and its frames are
    /// somebody else's now.
    fn exit(&mut self, handle: u64) {
        self.live.retain(|&h| h != handle);
        if let Some((px, ..)) = self.memory.get_mut(&handle) {
            px.fill(FOREIGN);
        }
    }

    /// The handle number comes round again for a new surface in fresh
    /// memory, and the old memory stays somebody else's.
    fn reissue(&mut self, handle: u64, w: u32, h: u32, colour: u32) {
        let old = self.memory.remove(&handle);
        self.register(handle, w, h, w, colour);
        if let Some(old) = old {
            self.memory.insert(handle | 1 << 63, old);
        }
    }
}

impl SurfaceKernel for Kernel {
    fn attach(&mut self, handle: u64) -> Option<Surface> {
        if !self.live.contains(&handle) {
            return None;
        }
        let (px, w, h, stride_px) = self.memory.get(&handle)?;
        Some(Surface {
            base_va: px.as_ptr() as u64,
            stride: stride_px * 4,
            width: *w,
            height: *h,
            byte_len: (*stride_px as u64) * (*h as u64) * 4,
        })
    }

    fn release(&mut self, _handle: u64) -> bool {
        true
    }
}

struct Desk {
    canvas: Vec<u32>,
    scene: SceneTable,
    attach: AttachCache,
    kernel: Kernel,
    damage: DamageAccumulator,
    cursor: (u32, u32),
}

impl Desk {
    fn new() -> Desk {
        let mut d = Desk {
            canvas: vec![0; (W * H) as usize],
            scene: SceneTable::new(),
            attach: AttachCache::new(),
            kernel: Kernel::default(),
            damage: DamageAccumulator::new(),
            cursor: (0, 0),
        };
        d.kernel.register(1, W, H, W, WALL);
        d.submit(1, 1, (0, 0, W, H), 0);
        d
    }

    fn dst(&mut self) -> Surface {
        Surface {
            base_va: self.canvas.as_mut_ptr() as u64,
            stride: W * 4,
            width: W,
            height: H,
            byte_len: (W * H * 4) as u64,
        }
    }

    fn submit(&mut self, owner: u32, handle: u64, r: (u32, u32, u32, u32), z: u32) {
        let l = Layer {
            owner_pid: owner,
            surface_handle: handle,
            x: r.0,
            y: r.1,
            width: r.2,
            height: r.3,
            z,
            stack: 0,
            in_use: true,
            miss_count: 0,
        };
        for rect in submit_layer(&mut self.scene, l).expect("room").into_iter().flatten() {
            self.damage.accumulate(rect);
        }
    }

    /// One frame: every damaged rectangle drained and composed.
    fn frame(&mut self) {
        while let Some(r) = self.damage.drain() {
            let dst = self.dst();
            let s = Scene {
                scene: &mut self.scene,
                attach: &mut self.attach,
                kernel: &mut self.kernel,
                damage: &mut self.damage,
            };
            compose(dst, r, s, Some(self.cursor));
        }
    }

    /// The pointer moves, damaging where it was and where it is, as the
    /// cursor-update handler does.
    fn move_cursor(&mut self, x: u32, y: u32) {
        for (cx, cy) in [self.cursor, (x, y)] {
            let w = CURSOR_SIDE.min(W - cx);
            let h = CURSOR_SIDE.min(H - cy);
            self.damage.accumulate(Rect { x: cx, y: cy, width: w, height: h });
        }
        self.cursor = (x, y);
    }

    /// The screen composed from scratch from what the scene holds now.
    fn reference(&mut self) -> Vec<u32> {
        let mut fresh = vec![0u32; (W * H) as usize];
        let dst = Surface {
            base_va: fresh.as_mut_ptr() as u64,
            stride: W * 4,
            width: W,
            height: H,
            byte_len: (W * H * 4) as u64,
        };
        let mut attach = AttachCache::new();
        let mut damage = DamageAccumulator::new();
        let s = Scene {
            scene: &mut self.scene,
            attach: &mut attach,
            kernel: &mut self.kernel,
            damage: &mut damage,
        };
        compose(dst, Rect { x: 0, y: 0, width: W, height: H }, s, Some(self.cursor));
        fresh
    }

    fn assert_clean(&mut self, what: &str) {
        if let Some(i) = self.canvas.iter().position(|&p| p == FOREIGN) {
            panic!(
                "{what}: pixel ({}, {}) came from another process's memory",
                i as u32 % W,
                i as u32 / W
            );
        }
    }

    fn assert_current(&mut self, what: &str) {
        let want = self.reference();
        if let Some(i) = (0..want.len()).find(|&i| self.canvas[i] != want[i]) {
            let (x, y) = (i as u32 % W, i as u32 / W);
            panic!(
                "{what}: ({x}, {y}) shows {:#010x}, a full frame {:#010x}",
                self.canvas[i], want[i]
            );
        }
    }

    /// The pointer crosses the screen in a staircase, a frame each step.
    fn staircase(&mut self, what: &str) {
        for step in 0..12 {
            self.move_cursor((8 + step * 6).min(W - 1), (4 + step * 5).min(H - 1));
            self.frame();
            self.assert_clean(what);
        }
    }
}

#[test]
fn a_window_whose_owner_exited_never_shows_reused_memory() {
    let mut d = Desk::new();
    d.kernel.register(20, 50, 36, 50, WINDOW);
    d.submit(2, 20, (20, 12, 50, 36), 2);
    d.frame();
    d.assert_current("before the exit");
    // The guest closes and its process ends with no scene remove.
    d.kernel.exit(20);
    d.staircase("after the exit");
    assert!(d.scene.layers().all(|l| l.owner_pid != 2), "the dead window's layer went");
    d.assert_current("after the exit");
}

#[test]
fn a_handle_number_given_to_another_owner_shows_only_the_new_surface() {
    let mut d = Desk::new();
    d.kernel.register(20, 50, 36, 50, WINDOW);
    d.submit(2, 20, (20, 12, 50, 36), 2);
    d.frame();
    d.kernel.exit(20);
    d.staircase("after the exit");
    // Another process registers a surface and the kernel hands it the same
    // handle number; the compositor's old mapping of it must not be used.
    d.kernel.reissue(20, 30, 20, OTHER);
    d.submit(4, 20, (50, 30, 30, 20), 2);
    d.frame();
    d.staircase("after the reuse");
    d.assert_current("after the reuse");
    assert_eq!(d.canvas[(40 * W + 60) as usize], OTHER, "the new window is drawn");
}

#[test]
fn a_layer_larger_than_its_surface_reads_nothing_past_it() {
    let mut d = Desk::new();
    // A resize in flight: the layer is submitted at the new size while the
    // surface is still the old, smaller one, with row padding in its stride.
    d.kernel.register(30, 20, 12, 28, WINDOW);
    d.submit(3, 30, (10, 10, 60, 40), 2);
    d.frame();
    d.assert_clean("a short surface");
    d.staircase("a short surface");
    d.assert_current("a short surface");
    assert_eq!(d.canvas[(15 * W + 15) as usize], WINDOW);
    assert_eq!(d.canvas[(45 * W + 60) as usize], WALL, "past the surface the wallpaper shows");
}

#[test]
fn the_cursor_over_changing_content_leaves_no_stale_block() {
    let mut d = Desk::new();
    d.kernel.register(40, 60, 40, 60, WINDOW);
    d.submit(5, 40, (10, 10, 60, 40), 2);
    d.frame();
    for step in 0..10u32 {
        // The app draws a new line under the pointer and commits it.
        let colour = 0xFF00_0000 | (step * 0x0011_1111);
        if let Some((px, ..)) = d.kernel.memory.get_mut(&40) {
            let row = 5 + step * 3;
            for y in row..row + 3 {
                for x in 0..60 {
                    px[(y * 60 + x) as usize] = colour;
                }
            }
            d.damage.accumulate(Rect { x: 10, y: 10 + row, width: 60, height: 3 });
        }
        d.move_cursor(14 + step * 4, 12 + step * 3);
        d.frame();
        d.assert_current("the pointer over a changing window");
    }
}

/// An app window's shadow and frame are drawn inside its own surface, in a
/// translucent margin round the frame, so its layer rectangle holds all of
/// it. A window moved over another, and the other raised over it, must leave
/// no band of its shadow or frame anywhere, from the damage the submit and
/// the raise report alone (no extra commit from the app).
#[test]
fn a_moved_and_raised_shadowed_window_leaves_no_band() {
    const SHADOW: u32 = 0x6000_0000;
    let mut d = Desk::new();
    d.kernel.register(50, 40, 30, 40, 0xFF20_2830);
    d.submit(6, 50, (40, 20, 40, 30), 2);
    // Window 7: a frame inside a 3 pixel shadow margin.
    d.kernel.register(60, 36, 28, 36, SHADOW);
    if let Some((px, ..)) = d.kernel.memory.get_mut(&60) {
        for y in 3..25 {
            for x in 3..33 {
                px[(y * 36 + x) as usize] = WINDOW;
            }
        }
    }
    d.submit(7, 60, (10, 10, 36, 28), 2);
    d.frame();
    d.assert_current("opened");
    // Dragged in steps across window 6, a submit at each new place.
    for (x, y) in [(18, 14), (27, 19), (36, 24), (45, 28), (52, 30)] {
        d.submit(7, 60, (x, y, 36, 28), 2);
        d.frame();
        d.assert_current("dragged");
    }
    // Window 6 is clicked: the window manager raises it.
    if let Some(r) = crate::scene_raise::raise_by_pid(&mut d.scene, 6) {
        d.damage.accumulate(r);
    }
    d.frame();
    d.assert_current("the window under it raised");
}
