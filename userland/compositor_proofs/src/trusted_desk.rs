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

//! A screen in memory for the trusted-band proofs: a full-screen window and
//! the shell's prompt over its middle in the trusted band, composed for the
//! screen and captured for a screenshot by the compositor's own code.

use crate::attach::AttachCache;
use crate::damage::{DamageAccumulator, Rect};
use crate::frame_pacer::canvas_copy::capture;
use crate::frame_pacer::compose::{compose, Scene};
use crate::scene::{Layer, SceneTable};
use crate::trusted_mem::{surface, Mem};
use crate::trusted_rule::TRUSTED_Z;

pub const W: u32 = 16;
pub const H: u32 = 12;
pub const APP: u32 = 0xFF30_A040;
pub const PROMPT: u32 = 0xFFFF_FFFF;

/// A full-screen window, and the shell's prompt over its middle.
fn desk(mem: &mut Mem) -> SceneTable {
    let mut scene = SceneTable::new();
    for (owner, handle, x, y, w, h, z, colour) in
        [(40, 1, 0, 0, W, H, 2, APP), (7, 2, 4, 3, 8, 6, TRUSTED_Z, PROMPT)]
    {
        mem.0.insert(handle, (vec![colour; (w * h) as usize], w, h));
        let (owner_pid, surface_handle, width, height) = (owner, handle, w, h);
        let l = Layer {
            owner_pid,
            surface_handle,
            x,
            y,
            width,
            height,
            z,
            in_use: true,
            ..Layer::default()
        };
        scene.submit(l).unwrap();
    }
    scene
}

pub fn shoot(dw: u32, dh: u32, ceiling: u32) -> (Vec<u32>, Vec<u32>, (u32, u32)) {
    let mut mem = Mem::default();
    let mut scene = desk(&mut mem);
    let (mut a, mut d) = (AttachCache::new(), DamageAccumulator::new());
    let mut screen = vec![0u32; (W * H) as usize];
    let s =
        Scene { scene: &mut scene, attach: &mut a, kernel: &mut mem, damage: &mut d, ceiling: !0 };
    compose(surface(&mut screen, W, H), Rect { x: 0, y: 0, width: W, height: H }, s, Some((0, 0)));
    let mut shot = vec![0xEEu32; (dw * dh) as usize];
    let s = Scene { scene: &mut scene, attach: &mut a, kernel: &mut mem, damage: &mut d, ceiling };
    let got = capture(surface(&mut shot, dw, dh), (W, H), s);
    (screen, shot, got)
}
