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

//! Moves, buffer swaps and closes that land faster than frames, as a drag
//! at KVM speed sends them: many between two composites, the damage drained
//! through the shipped batching (frame_pacer/drain_batch.rs), a batch the
//! display refused damaged again. Every pixel a window left must show what
//! is under it once the frame after the burst is composed.

use super::multi_rig::keyed;
use super::repaint_rig::*;
use crate::damage::{DamageAccumulator, Rect};
use crate::drain_batch::drain_batched;
use crate::scene::SceneTable;
use crate::scene_remove::{remove_by_pid, remove_layer};
use crate::scene_submit::submit_keyed_layer;
use crate::state::present_rule::{present, present_on};
use crate::tests::scene::frame_model::{layer, Rng, Screen, BACKGROUND, H, W};

const DESK: u32 = 1;
const APP_Z: u32 = 2;

struct Desk {
    scene: SceneTable,
    acc: DamageAccumulator,
    screen: Screen,
}

impl Desk {
    /// A wallpaper over the whole screen, a shell strip on top, composed.
    fn new() -> Desk {
        let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
        open(&mut scene, &mut acc, DESK, (0, 0, W, H));
        let strip = layer(2, 0, 0, W, 4, 3);
        submit(&mut scene, &mut acc, strip);
        let mut d = Desk { screen: Screen::composed(&scene), scene, acc };
        d.frame(None);
        d
    }

    /// One frame: the damage drained in batches; the batch numbered `refuse`
    /// is refused by the display, damaged again, and the next frame sends it.
    fn frame(&mut self, refuse: Option<usize>) {
        let mut ctx = (&mut self.scene, &mut self.acc, &mut self.screen, 0usize);
        let refused = drain_batched(
            &mut ctx,
            |c| &mut *c.1,
            |c, r| {
                c.2.paint(c.0, r);
                Some(r)
            },
            |c, _shown: &[Rect]| {
                c.3 += 1;
                match refuse == Some(c.3 - 1) {
                    true => Err(()),
                    false => Ok(()),
                }
            },
        );
        if refused.is_err() {
            // Composed again whole next frame: the screen keeps what was sent.
            self.frame(None);
        }
    }

    fn check(&self, what: &str) {
        assert_current(&self.screen, &self.scene, what);
    }
}

#[test]
fn a_burst_of_moves_between_two_frames_leaves_no_pixel_behind() {
    for seed in 1..=64u64 {
        let mut rng = Rng::new(seed);
        let mut d = Desk::new();
        let (w, h) = (8 + rng.below(W / 3), 8 + rng.below(H / 3));
        let mut at = (rng.below(W - w), rng.below(H - h));
        let mut handle = 700u64;
        let win = |handle: u64, (x, y): (u32, u32)| crate::scene::Layer {
            surface_handle: handle,
            ..layer(7, x, y, w, h, APP_Z)
        };
        submit(&mut d.scene, &mut d.acc, win(handle, at));
        d.frame(None);
        for burst in 0..12 {
            // Up to 40 moves of a few pixels, a swapchain present among them.
            for _ in 0..1 + rng.below(40) {
                at = (
                    (at.0 + rng.below(7)).saturating_sub(3).min(W - w),
                    (at.1 + rng.below(7)).saturating_sub(3).min(H - h),
                );
                submit(&mut d.scene, &mut d.acc, win(handle, at));
                if rng.below(4) == 0 {
                    handle ^= 1;
                    let changed = Rect { x: 0, y: 0, width: w, height: h };
                    let s = present(&mut d.scene, 7, APP_Z, handle, changed).expect("presents");
                    d.acc.accumulate(s.repaint.expect("on the layer"));
                }
            }
            let refuse = (rng.below(3) == 0).then(|| rng.below(2) as usize);
            d.frame(refuse);
            d.check(&format!("seed {seed}, burst {burst}"));
        }
        // Moved again and closed before the next frame.
        at = (W - w, H - h);
        submit(&mut d.scene, &mut d.acc, win(handle, at));
        let gone = remove_by_pid(&mut d.scene, 7).expect("held");
        d.acc.accumulate(gone);
        d.frame(None);
        d.check(&format!("seed {seed}, closed"));
        assert!(d.screen.px.iter().all(|&p| p != 7), "seed {seed}: the closed window shows");
    }
}

#[test]
fn keyed_windows_of_one_client_moved_swapped_and_closed_in_one_frame() {
    for seed in 1..=64u64 {
        let mut rng = Rng::new(seed);
        let mut d = Desk::new();
        let mut shown = [801u64, 802];
        let mut rects = [(2u32, 6u32, 20u32, 14u32), (30, 20, 18, 16)];
        for (i, r) in rects.iter().enumerate() {
            let l = keyed(40, 1 + i as u32, shown[i], *r);
            for rect in submit_keyed_layer(&mut d.scene, l, None)
                .expect("room")
                .repaint
                .into_iter()
                .flatten()
            {
                d.acc.accumulate(rect);
            }
        }
        d.frame(None);
        for step in 0..60 {
            let i = rng.below(2) as usize;
            match rng.below(3) {
                0 => {
                    let r = &mut rects[i];
                    r.0 = rng.below(W - r.2);
                    r.1 = rng.below(H - r.3);
                    let l = keyed(40, 1 + i as u32, shown[i], *r);
                    let done = submit_keyed_layer(&mut d.scene, l, None).expect("moves");
                    done.repaint.into_iter().flatten().for_each(|x| d.acc.accumulate(x));
                }
                1 => {
                    let next = shown[i] + 10;
                    let all = Rect { x: 0, y: 0, width: rects[i].2, height: rects[i].3 };
                    let s =
                        present_on(&mut d.scene, 40, APP_Z, shown[i], next, all).expect("names it");
                    d.acc.accumulate(s.repaint.expect("on the layer"));
                    shown[i] = next;
                }
                _ => {}
            }
            if rng.below(3) == 0 {
                d.frame(None);
                d.check(&format!("seed {seed}, step {step}"));
            }
        }
        // Both closed in one frame.
        for h in shown {
            if let Some(r) = remove_layer(&mut d.scene, 40, APP_Z, h) {
                d.acc.accumulate(r.rect);
            }
        }
        d.frame(None);
        d.check(&format!("seed {seed}, both closed"));
        assert!(d.screen.px.iter().all(|&p| p == DESK || p == 2 || p == BACKGROUND));
    }
}
