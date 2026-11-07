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

//! A full attach cache evicts a surface no layer uses, and one full of live
//! layers refuses rather than evict a surface still drawn.

use super::attach_kernel::*;
use crate::attach::{AttachCache, MAX_ATTACH};

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

/// A window resized takes new buffers and lets its old ones go once the
/// compositor shows the new. The old buffer that was not on screen was still
/// mapped here, its frames kept for this process until the cache filled: a
/// whole buffer held for every resize. One composite's sweep lets such a
/// mapping go once the kernel no longer has its surface, asks the kernel of
/// one slot a call, and keeps a mapping still alive or still drawn.
#[test]
fn a_mapping_no_layer_uses_is_let_go_once_its_surface_is_gone() {
    let mut cache = AttachCache::new();
    let mut kernel = FakeKernel::default();
    // A window's two buffers, 1 shown and 2 shown before it, and a new
    // window's 3.
    kernel.live.extend([1, 2, 3]);
    for h in [1, 2, 3] {
        let _ = cache.lookup(h, &[1, 2, 3], &mut kernel);
    }
    let in_scene = [3];
    // Resized: 1 and 2 let go by their owner, 3 on screen.
    kernel.live.remove(&1);
    kernel.live.remove(&2);
    let before = kernel.attaches;
    assert!(cache.sweep_one(&in_scene, &mut kernel));
    assert_eq!(kernel.attaches, before + 1, "one surface asked of");
    assert!(cache.sweep_one(&in_scene, &mut kernel));
    assert!(!cache.sweep_one(&in_scene, &mut kernel), "nothing left to let go");
    let mut released = kernel.released.clone();
    released.sort_unstable();
    assert_eq!(released, vec![1, 2]);
    assert_eq!(kernel.mapped.iter().copied().collect::<Vec<_>>(), vec![3], "the drawn one kept");

    // A buffer no layer draws from now but still alive (a swapchain's other
    // one) is kept, and asked of again only after the others.
    kernel.live.insert(4);
    let _ = cache.lookup(4, &[3, 4], &mut kernel);
    assert!(!cache.sweep_one(&in_scene, &mut kernel));
    assert!(kernel.mapped.contains(&4));
    assert!(matches!(cache.lookup(4, &[4], &mut kernel), crate::attach::Attached::Live(_)));
}

/// Each composite sweeps: the frame that follows a resize lets go of the old
/// buffer it still mapped, and keeps the layer's own.
#[test]
fn a_composite_lets_go_of_a_buffer_its_owner_let_go() {
    use crate::damage::DamageAccumulator;
    use crate::scene::{Layer, SceneTable};
    use crate::sw_blitter::Surface;
    use crate::visible::visible_layers;
    let shown = Layer {
        owner_pid: 9,
        surface_handle: 20,
        z: 2,
        width: 8,
        height: 8,
        in_use: true,
        ..Layer::default()
    };
    let mut scene = SceneTable::new();
    scene.submit(shown).expect("room");
    let (mut cache, mut kernel, mut acc) =
        (AttachCache::new(), FakeKernel::default(), DamageAccumulator::new());
    kernel.live.extend([20, 21]);
    let _ = cache.lookup(21, &[20, 21], &mut kernel);
    kernel.live.remove(&21);
    let mut out = [(Layer::default(), Surface::default(), None); MAX_ATTACH];
    assert_eq!(visible_layers(&mut scene, &mut cache, &mut kernel, &mut acc, &mut out), 1);
    assert_eq!(kernel.released, vec![21], "the old buffer let go");
    assert!(kernel.mapped.contains(&20), "the shown one kept");
}
