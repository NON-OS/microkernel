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

//! A window whose owner exits without a scene remove leaves the screen at
//! once, and the compositor never draws from a mapping the kernel has taken
//! back. The kernel here keeps the set of surfaces it still knows, as the
//! surface registry does: an owner's exit drops every record of its surfaces.

use std::collections::HashSet;

use crate::attach::{AttachCache, SurfaceKernel, MAX_ATTACH};
use crate::damage::{DamageAccumulator, Rect};
use crate::frame_model::{layer, Screen};
use crate::scene::{Layer, SceneTable};
use crate::scene_submit::submit_layer;
use crate::sw_blitter::Surface;
use crate::visible::visible_layers;

#[derive(Default)]
struct FakeKernel {
    live: HashSet<u64>,
    /// Handles whose release the kernel refuses (the compositor holds none).
    refuse: HashSet<u64>,
    attaches: usize,
    released: Vec<u64>,
    /// The views the compositor holds. An attach the kernel already holds
    /// answers from it; a new one maps the surface again (`maps`). A view
    /// of a surface whose owner exited keeps its frames (an orphan) until
    /// it is released.
    mapped: HashSet<u64>,
    maps: usize,
}

impl SurfaceKernel for FakeKernel {
    fn attach(&mut self, handle: u64) -> Option<Surface> {
        self.attaches += 1;
        if !self.live.contains(&handle) {
            return None;
        }
        if self.mapped.insert(handle) {
            self.maps += 1;
        }
        self.live.contains(&handle).then_some(Surface {
            base_va: 0x1000 * handle,
            stride: 4,
            width: 1,
            height: 1,
            byte_len: 4,
        })
    }

    fn release(&mut self, handle: u64) -> bool {
        if self.refuse.contains(&handle) {
            return false;
        }
        self.mapped.remove(&handle);
        self.released.push(handle);
        true
    }
}

/// One frame as `frame_pacer::tick` runs it: every damaged rectangle drained
/// and composed from the layers `visible_layers` hands back, including the
/// rectangles it damages itself while composing.
fn frame(
    screen: &mut Screen,
    scene: &mut SceneTable,
    cache: &mut AttachCache,
    kernel: &mut FakeKernel,
    acc: &mut DamageAccumulator,
) {
    while let Some(r) = acc.drain() {
        let mut out = [(Layer::default(), Surface::default()); MAX_ATTACH];
        let n = visible_layers(scene, cache, kernel, acc, &mut out);
        let layers: Vec<Layer> = out[..n].iter().map(|(l, _)| *l).collect();
        screen.paint_layers(&layers, r);
    }
}

fn open(scene: &mut SceneTable, acc: &mut DamageAccumulator, owner: u32, r: (u32, u32, u32, u32)) {
    let l = layer(owner, r.0, r.1, r.2, r.3, 2);
    for rect in submit_layer(scene, l).expect("room for the layer").into_iter().flatten() {
        acc.accumulate(rect);
    }
}

#[test]
fn a_window_whose_owner_died_leaves_the_screen_in_the_next_frame() {
    let (mut scene, mut acc, mut cache) =
        (SceneTable::new(), DamageAccumulator::new(), AttachCache::new());
    let mut kernel = FakeKernel::default();
    kernel.live.extend([1, 2]);
    open(&mut scene, &mut acc, 1, (2, 2, 40, 30));
    open(&mut scene, &mut acc, 2, (20, 14, 40, 30));
    let mut screen = Screen::composed(&scene);
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    assert_eq!(screen.top_at(30, 20), 2);

    // Owner 2 crashes: the kernel forgets its surface, and the compositor
    // only learns of it from its own lookups. The cursor moves far away.
    kernel.live.remove(&2);
    acc.accumulate(Rect { x: 0, y: 44, width: 4, height: 4 });
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);

    assert!(scene.layers().all(|l| l.owner_pid != 2), "the dead window's layer is gone");
    let full = Screen::composed(&scene);
    if let Some((x, y, shown, wanted)) = screen.first_difference(&full) {
        panic!("pixel ({x}, {y}) still shows owner {shown}, a full frame shows {wanted}");
    }
    assert_eq!(screen.top_at(30, 20), 1, "the window under it shows again");
}

#[test]
fn a_mapped_surface_is_checked_with_the_kernel_on_every_composite() {
    let (mut scene, mut acc, mut cache) =
        (SceneTable::new(), DamageAccumulator::new(), AttachCache::new());
    let mut kernel = FakeKernel::default();
    kernel.live.insert(5);
    open(&mut scene, &mut acc, 5, (0, 0, 8, 8));
    let mut out = [(Layer::default(), Surface::default()); MAX_ATTACH];
    for round in 1..=3 {
        assert_eq!(visible_layers(&mut scene, &mut cache, &mut kernel, &mut acc, &mut out), 1);
        assert_eq!(kernel.attaches, round, "composite {round} asked the kernel");
    }
    kernel.live.remove(&5);
    assert_eq!(visible_layers(&mut scene, &mut cache, &mut kernel, &mut acc, &mut out), 0);
    assert_eq!(kernel.released, vec![5], "the dead surface is released, freeing its frames");
}

#[test]
fn a_surface_that_never_attached_waits_for_the_reaper() {
    let (mut scene, mut acc, mut cache) =
        (SceneTable::new(), DamageAccumulator::new(), AttachCache::new());
    let mut kernel = FakeKernel::default();
    open(&mut scene, &mut acc, 7, (0, 0, 8, 8));
    let mut out = [(Layer::default(), Surface::default()); MAX_ATTACH];
    let _ = visible_layers(&mut scene, &mut cache, &mut kernel, &mut acc, &mut out);
    assert_eq!(scene.layers().count(), 1, "one missed attach drops nothing");
}

#[test]
fn a_refused_release_still_frees_the_slot() {
    let mut cache = AttachCache::new();
    let mut kernel = FakeKernel::default();
    // More handles than the cache has slots, each released by a kernel that
    // no longer holds a reference for the compositor.
    for h in 1..=(MAX_ATTACH as u64 + 4) {
        kernel.live.insert(h);
        kernel.refuse.insert(h);
        let _ = cache.lookup(h, &[], &mut kernel);
        assert!(!cache.forget(h, &mut kernel));
    }
    // A fresh surface must still get a slot, or its reference would never be
    // given back when its window goes.
    let fresh = 1000;
    kernel.live.insert(fresh);
    let _ = cache.lookup(fresh, &[], &mut kernel);
    assert!(cache.forget(fresh, &mut kernel));
    assert_eq!(kernel.released, vec![fresh]);
}

/// An app that exits holding its window (no scene remove) leaves the
/// compositor's view of it as the last hold on its frames. The compositor
/// must let go in the frame that finds the owner gone, exactly once, or the
/// window's memory (8 MB for a 1080p window) is never freed.
#[test]
fn an_exited_apps_window_memory_is_let_go_at_once() {
    let (mut scene, mut acc, mut cache) =
        (SceneTable::new(), DamageAccumulator::new(), AttachCache::new());
    let mut kernel = FakeKernel::default();
    kernel.live.extend([1, 2, 3]);
    for (owner, r) in [(1, (0, 0, 20, 20)), (2, (10, 10, 20, 20)), (3, (30, 5, 20, 20))] {
        open(&mut scene, &mut acc, owner, r);
    }
    let mut screen = Screen::composed(&scene);
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    assert_eq!(kernel.mapped.len(), 3);
    kernel.live.remove(&2);
    for _ in 0..5 {
        acc.accumulate(Rect { x: 0, y: 44, width: 4, height: 4 });
        frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    }
    assert!(!kernel.mapped.contains(&2), "the dead window's frames are released");
    assert_eq!(kernel.released, vec![2], "released once, nothing else");
    assert!(scene.layers().all(|l| l.owner_pid != 2));
}

/// A normal close: the client's scene remove forgets its surface, and a
/// later lookup of anything maps nothing stale.
#[test]
fn a_closed_window_is_forgotten_and_released() {
    let mut cache = AttachCache::new();
    let mut kernel = FakeKernel::default();
    kernel.live.insert(9);
    let _ = cache.lookup(9, &[9], &mut kernel);
    assert!(cache.forget(9, &mut kernel));
    assert!(kernel.mapped.is_empty());
    let _ = cache.forget(9, &mut kernel);
    assert_eq!(kernel.released, vec![9], "released once, a second forget releases nothing");
}

/// A full cache used to map a new surface afresh on every composite and keep
/// none of it, a mapping leaked each frame. It now lets go of the least
/// recently used surface no layer is drawn from, and keeps the new one.
#[test]
fn a_full_cache_evicts_a_surface_no_layer_uses() {
    let mut cache = AttachCache::new();
    let mut kernel = FakeKernel::default();
    let stale: Vec<u64> = (1..=MAX_ATTACH as u64).collect();
    kernel.live.extend(stale.iter().copied());
    for &h in &stale {
        let _ = cache.lookup(h, &stale, &mut kernel);
    }
    // Every layer but the first went; surface 1 is the least recently used.
    let new = 500;
    kernel.live.insert(new);
    let in_scene: Vec<u64> = stale[1..].iter().copied().chain([new]).collect();
    let maps_before = kernel.maps;
    for _ in 0..10 {
        assert!(matches!(
            cache.lookup(new, &in_scene, &mut kernel),
            crate::attach::Attached::Live(_)
        ));
    }
    assert_eq!(kernel.maps, maps_before + 1, "mapped once, then kept");
    assert_eq!(kernel.released, vec![1], "the surface no layer used was let go");
    assert_eq!(kernel.mapped.len(), MAX_ATTACH);
}

/// With every slot serving a layer of the scene there is nothing to evict:
/// the lookup says so and maps nothing.
#[test]
fn a_full_cache_of_live_layers_refuses_honestly() {
    let mut cache = AttachCache::new();
    let mut kernel = FakeKernel::default();
    let live: Vec<u64> = (1..=MAX_ATTACH as u64).collect();
    kernel.live.extend(live.iter().copied());
    for &h in &live {
        let _ = cache.lookup(h, &live, &mut kernel);
    }
    kernel.live.insert(77);
    let attaches = kernel.attaches;
    let mut with_new = live.clone();
    with_new.push(77);
    assert!(matches!(cache.lookup(77, &with_new, &mut kernel), crate::attach::Attached::NoRoom));
    assert_eq!(kernel.attaches, attaches, "nothing was mapped");
    assert!(kernel.released.is_empty());
}
