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

//! A client on `Chain` and the compositor's own present and release rules
//! run against each other over random sequences, the kernel's release
//! counts modelled. Two things hold throughout: the client is never handed
//! a buffer the compositor shows or still holds, and the buffer a layer
//! shows always holds the whole current picture (what the client did not
//! redraw, nothing presented since had changed).

use crate::damage::Rect;
use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::state::present_rule::present;
use crate::state::retire::Retiring;
use crate::swapchain::swapchain::Chain;
use crate::swapchain::swapchain_damage::{Damage, Stale};
use crate::tests::scene::frame_util::{layer, Rng};

const W: u32 = 64;
const OWNER: u32 = 7;
const Z: u32 = 2;

struct World {
    scene: SceneTable,
    retiring: Retiring,
    /// What the compositor holds, mirrored from `hold` and the releases.
    held: Vec<u64>,
    chain: Chain,
    handles: Vec<u64>,
    pixels: Vec<Vec<u32>>,
    fence: Vec<u32>,
    truth: Vec<u32>,
    frame: u32,
}

impl World {
    fn new(count: usize) -> World {
        let handles: Vec<u64> = (0..count as u64).map(|i| 100 + i).collect();
        // The first buffer is drawn whole and submitted with the layer.
        let mut chain = Chain::new(count).expect("2 or 3");
        assert_eq!(chain.acquire(), Some((0, Stale::Whole)));
        chain.present(0, Damage { x: 0, y: 0, width: W, height: 1 }, |_| 0).expect("drawing");
        let mut scene = SceneTable::new();
        let mut l = layer(OWNER, 0, 0, W, 1, Z);
        l.surface_handle = handles[0];
        submit_layer(&mut scene, l).expect("room");
        let (retiring, held) = (Retiring::new(), Vec::new());
        let (pixels, fence) = (vec![vec![0; W as usize]; count], vec![0; count]);
        World {
            scene,
            retiring,
            held,
            chain,
            handles,
            pixels,
            fence,
            truth: vec![0; W as usize],
            frame: 0,
        }
    }

    fn shown(&self) -> u64 {
        self.scene.layers().find(|l| l.owner_pid == OWNER).map_or(0, |l| l.surface_handle)
    }

    /// Take a buffer, bring it up to date, draw `d` as a new frame and
    /// present it. None when every buffer is shown or held.
    fn draw(&mut self, d: Damage) -> Option<usize> {
        let (i, stale) = self.chain.acquire()?;
        let h = self.handles[i];
        assert!(
            h != self.shown() && !self.held.contains(&h),
            "handed a buffer the compositor may read"
        );
        let (from, to) = match stale {
            Stale::Whole => (0, W),
            Stale::Box(b) => (b.x, b.x + b.width),
            Stale::None => (0, 0),
        };
        for x in from..to {
            self.pixels[i][x as usize] = self.truth[x as usize];
        }
        self.frame += 1;
        for x in d.x..d.x + d.width {
            self.truth[x as usize] = self.frame;
            self.pixels[i][x as usize] = self.frame;
        }
        let fences = self.fence.clone();
        self.chain.present(i, d, |j| fences[j]).expect("was drawing");
        let r = Rect { x: d.x, y: 0, width: d.width, height: d.height };
        let s = present(&mut self.scene, OWNER, Z, h, r).expect("our layer");
        if let Some(old) = s.old {
            assert!(self.retiring.hold(old), "room");
            if !self.held.contains(&old) {
                self.held.push(old);
            }
        }
        Some(i)
    }

    /// A frame: the layer's buffer is read whole, then the held buffers no
    /// layer shows are released (the kernel's count moves on).
    fn compose(&mut self) {
        let i = self.handles.iter().position(|&h| h == self.shown()).expect("ours");
        assert_eq!(self.pixels[i], self.truth, "the shown buffer is the picture");
        let (scene, handles, fence, held) =
            (&self.scene, &self.handles, &mut self.fence, &mut self.held);
        let shown = |h: u64| scene.layers().any(|l| l.surface_handle == h);
        self.retiring.release_unshown(shown, |h| {
            let j = handles.iter().position(|&x| x == h).expect("ours");
            fence[j] += 1;
            held.retain(|&x| x != h);
        });
    }

    /// The client reads its retiring buffers' counts.
    fn poll(&mut self) {
        for j in 0..self.handles.len() {
            self.chain.fence(j, self.fence[j]);
        }
    }
}

fn damage(r: &mut Rng) -> Damage {
    let x = r.below(W);
    Damage { x, y: 0, width: 1 + r.below(W - x), height: 1 }
}

#[test]
fn no_buffer_is_drawn_while_the_compositor_may_read_it() {
    for count in [2, 3] {
        for seed in 1..=300 {
            let mut r = Rng::new(seed);
            let mut w = World::new(count);
            for _ in 0..60 {
                match r.below(4) {
                    0 | 1 => {
                        let _ = w.draw(damage(&mut r));
                    }
                    2 => w.compose(),
                    _ => w.poll(),
                }
            }
            w.compose();
        }
    }
}

#[test]
fn a_client_presenting_faster_than_frames_waits_for_a_release() {
    let mut w = World::new(2);
    // Buffer 0 is shown; 1 is drawn and presented, 0 retires.
    assert_eq!(w.draw(Damage { x: 0, y: 0, width: 4, height: 1 }), Some(1));
    assert_eq!(w.draw(Damage { x: 0, y: 0, width: 4, height: 1 }), None, "0 is still held");
    w.poll();
    assert_eq!(
        w.draw(Damage { x: 0, y: 0, width: 4, height: 1 }),
        None,
        "no frame yet: no release"
    );
    w.compose();
    w.poll();
    assert_eq!(
        w.draw(Damage { x: 8, y: 0, width: 4, height: 1 }),
        Some(0),
        "released after the frame"
    );
    w.compose();
}

#[test]
fn triple_buffering_draws_the_next_frame_while_one_is_held() {
    let mut w = World::new(3);
    assert_eq!(w.draw(Damage { x: 0, y: 0, width: 8, height: 1 }), Some(1));
    assert_eq!(
        w.draw(Damage { x: 4, y: 0, width: 8, height: 1 }),
        Some(2),
        "0 held, 1 held, 2 free"
    );
    assert_eq!(w.draw(Damage { x: 4, y: 0, width: 8, height: 1 }), None);
    w.compose();
    w.poll();
    assert!(w.draw(Damage { x: 0, y: 0, width: 2, height: 1 }).is_some());
    w.compose();
}

#[test]
fn a_buffer_taken_again_is_stale_by_what_the_others_presented() {
    let mut c = Chain::new(2).expect("two");
    let (a, _) = c.acquire().expect("free");
    c.present(a, Damage { x: 0, y: 0, width: 10, height: 10 }, |_| 0).expect("drawing");
    let (b, stale) = c.acquire().expect("free");
    assert_eq!(stale, Stale::Whole, "never drawn");
    c.present(b, Damage { x: 5, y: 5, width: 2, height: 2 }, |_| 0).expect("drawing");
    c.fence(a, 1);
    let (again, stale) = c.acquire().expect("released");
    assert_eq!(again, a);
    assert_eq!(stale, Stale::Box(Damage { x: 5, y: 5, width: 2, height: 2 }));
    assert!(c.present(b, Damage::default(), |_| 0).is_err(), "b is shown, not drawing");
}
