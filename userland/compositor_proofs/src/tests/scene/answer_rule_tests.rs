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

//! Whether a sender answers for a surface (state/answer_rule.rs), over a
//! kernel double that names each surface's owner and keeps the views this
//! process holds, as the surface registry does: an attach of a surface
//! already viewed answers from that view, a new one maps it again.

use std::collections::{HashMap, HashSet};

use crate::state::answer_rule::{answers, capture_target, Owners};
use crate::attach::{AttachCache, Attached, SurfaceKernel, MAX_ATTACH};
use crate::sw_blitter::Surface;

const CLIENT: u32 = 40;
const OTHER: u32 = 41;
/// A Linux guest's supervisor: the bridge, which answers for its pins.
const BRIDGE: u32 = 42;

#[derive(Default)]
struct Registry {
    /// Who answers for each surface: its owner, or a pin's supervisor.
    owners: HashMap<u64, u32>,
    mapped: HashSet<u64>,
    maps: usize,
    asked: usize,
    released: Vec<u64>,
    /// Each worker thread's process; any other pid is its own.
    threads: HashMap<u32, u32>,
}

impl Registry {
    fn with(surfaces: &[(u64, u32)]) -> Registry {
        Registry { owners: surfaces.iter().copied().collect(), ..Registry::default() }
    }

    fn view(&mut self, handle: u64) -> Option<u32> {
        self.asked += 1;
        let owner = *self.owners.get(&handle)?;
        if self.mapped.insert(handle) {
            self.maps += 1;
        }
        Some(owner)
    }

    fn drop_view(&mut self, handle: u64) {
        self.mapped.remove(&handle);
        self.released.push(handle);
    }
}

impl Owners for Registry {
    fn owner(&mut self, handle: u64) -> Option<u32> {
        self.view(handle)
    }

    fn release(&mut self, handle: u64) {
        self.drop_view(handle);
    }

    fn process_of(&mut self, sender: u32) -> u32 {
        self.threads.get(&sender).copied().unwrap_or(sender)
    }
}

impl SurfaceKernel for Registry {
    fn attach(&mut self, handle: u64) -> Option<Surface> {
        self.view(handle)?;
        Some(Surface { base_va: 0x1000 * handle, stride: 4, width: 1, height: 1, byte_len: 4 })
    }

    fn release(&mut self, handle: u64) -> bool {
        self.drop_view(handle);
        true
    }
}

#[test]
fn a_surface_the_cache_maps_is_asked_from_that_view_and_kept() {
    let mut k = Registry::with(&[(7, CLIENT)]);
    k.mapped.insert(7);
    assert!(answers(&mut k, false, true, CLIENT, 7));
    assert!(!answers(&mut k, false, true, OTHER, 7));
    assert_eq!((k.maps, k.released.len()), (0, 0), "no new view, none let go");
    assert!(k.mapped.contains(&7), "the cache's view is still there");
}

#[test]
fn a_surface_not_mapped_is_asked_through_a_view_let_go_at_once() {
    let mut k = Registry::with(&[(7, CLIENT)]);
    assert!(answers(&mut k, false, false, CLIENT, 7), "the client's own new buffer");
    assert_eq!(k.released, [7]);
    assert!(k.mapped.is_empty(), "nothing kept: the composite maps it when it draws it");
}

#[test]
fn a_guessed_handle_of_another_s_surface_is_refused_and_leaves_no_view() {
    let mut k = Registry::with(&[(7, OTHER)]);
    assert!(!answers(&mut k, false, false, CLIENT, 7));
    assert!(k.mapped.is_empty(), "no map of another's surface kept");
    let mut gone = Registry::default();
    assert!(!answers(&mut gone, false, false, CLIENT, 9), "a handle the kernel will not show");
    assert!(gone.released.is_empty(), "no view, so nothing to let go");
}

#[test]
fn a_view_of_the_screen_is_never_asked_nor_let_go() {
    let mut k = Registry::with(&[(3, CLIENT)]);
    k.mapped.insert(3);
    assert!(!answers(&mut k, true, false, CLIENT, 3), "the primary or a head");
    assert_eq!((k.asked, k.released.len()), (0, 0));
    assert!(k.mapped.contains(&3), "what the compositor draws into stays mapped");
    assert!(!answers(&mut k, false, false, CLIENT, 0));
    assert_eq!(k.asked, 0, "handle 0 names nothing");
}

#[test]
fn a_pinned_buffer_is_answered_for_by_its_guest_s_supervisor_alone() {
    let mut k = Registry::with(&[(11, BRIDGE)]);
    assert!(answers(&mut k, false, false, BRIDGE, 11));
    assert!(!answers(&mut k, false, false, CLIENT, 11));
}

/// A finding from a boot: with every slot serving a drawn layer,
/// the question took a slot first, got NoRoom and refused the client its
/// own new buffer; and a guessed handle took a slot, evicting an idle view.
#[test]
fn a_full_cache_refuses_no_client_its_own_buffer_and_loses_no_mapping() {
    let drawn: Vec<u64> = (1..=MAX_ATTACH as u64).collect();
    let mut owners: Vec<(u64, u32)> = drawn.iter().map(|&h| (h, OTHER)).collect();
    owners.extend([(500, CLIENT), (501, OTHER)]);
    let mut k = Registry::with(&owners);
    let mut cache = AttachCache::new();
    for &h in &drawn {
        assert!(matches!(cache.lookup(h, &drawn, &mut k), Attached::Live(_)));
    }
    let views = k.mapped.clone();
    let cached = cache.holds(500);
    assert!(!cached);
    assert!(answers(&mut k, false, cached, CLIENT, 500), "its own buffer, cache full or not");
    assert!(!answers(&mut k, false, cache.holds(501), CLIENT, 501), "another's, guessed");
    assert_eq!(k.mapped, views, "every drawn layer's view kept, nothing new kept");
    assert!(drawn.iter().all(|&h| cache.holds(h)), "no slot evicted");
    assert!(!k.released.iter().any(|h| drawn.contains(h)));
}

#[test]
fn holds_asks_nothing_of_the_kernel() {
    let mut k = Registry::with(&[(7, CLIENT)]);
    let mut cache = AttachCache::new();
    let _ = cache.lookup(7, &[7], &mut k);
    let asked = k.asked;
    assert!(cache.holds(7) && !cache.holds(8) && !cache.holds(0));
    assert_eq!(k.asked, asked);
}

#[test]
fn a_capture_goes_only_into_the_sender_s_own_surface_not_viewed() {
    let mut k = Registry::with(&[(7, CLIENT), (8, OTHER)]);
    assert!(capture_target(&mut k, false, false, CLIENT, 7), "its own new surface");
    assert_eq!(k.released, [7], "the question's view let go before the capture's");
    assert!(!capture_target(&mut k, false, false, CLIENT, 8), "another's surface");
    assert!(!capture_target(&mut k, false, false, CLIENT, 9), "one the kernel will not show");
    assert!(!capture_target(&mut k, false, false, CLIENT, 0));
    assert!(k.mapped.is_empty());
}

/// A capture's attach of a surface the compositor views hands back that view
/// and its release unmaps it: a window the cache draws from, or the screen.
#[test]
fn a_capture_never_goes_into_a_surface_the_compositor_views() {
    let mut k = Registry::with(&[(7, CLIENT), (3, CLIENT)]);
    k.mapped.extend([7, 3]);
    assert!(!capture_target(&mut k, false, true, CLIENT, 7), "a window in the cache");
    assert!(!capture_target(&mut k, true, false, CLIENT, 3), "the primary or a head");
    assert_eq!(k.released.len(), 0, "no view let go");
    assert!(k.mapped.contains(&7) && k.mapped.contains(&3));
}

#[test]
fn a_clients_worker_thread_answers_for_its_process_and_no_other() {
    const WORKER: u32 = 60;
    const OTHERS_WORKER: u32 = 61;
    let mut k = Registry::with(&[(7, CLIENT)]);
    k.threads = [(WORKER, CLIENT), (OTHERS_WORKER, OTHER)].into_iter().collect();
    assert!(answers(&mut k, false, false, WORKER, 7), "a present from a worker thread");
    assert!(!answers(&mut k, false, false, OTHERS_WORKER, 7), "another process's thread");
    assert!(capture_target(&mut k, false, false, WORKER, 7));
    assert!(!capture_target(&mut k, false, false, OTHERS_WORKER, 7));
}
