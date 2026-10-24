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
