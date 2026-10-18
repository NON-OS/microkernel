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

//! A surface that never attached waits for the reaper, and a release the
//! kernel refuses still frees the cache slot.

use super::attach_kernel::*;
use super::attach_rig::*;
use crate::attach::{AttachCache, MAX_ATTACH};
use crate::damage::DamageAccumulator;
use crate::scene::{Layer, SceneTable};
use crate::sw_blitter::Surface;
use crate::visible::visible_layers;

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
