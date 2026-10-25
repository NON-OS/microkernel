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

//! K15b: a grant mapped by one thread, a surface registered by another
//! thread of the same process, are matched by process.

use super::common::{grant_frames, Registry, COMPOSITOR, FB, GPU, LEN, START};
use super::dma_rule::PageOrphan;

/// The GPU capsule's worker thread, which maps the framebuffer grant.
const WORKER: u32 = 8;

fn two_threads() -> Registry {
    Registry {
        attaches: vec![(FB, COMPOSITOR)],
        threads: vec![(WORKER, GPU)],
        ..Default::default()
    }
}

#[test]
fn a_workers_grant_under_the_main_threads_surface_is_held() {
    let mut r = two_threads();
    assert!(r.release(WORKER, vec![(FB, GPU, grant_frames())]), "found by process");
    assert!(r.given_back.is_empty(), "nothing freed under the compositor");
    assert_eq!(r.orphans[0].owner, GPU, "the orphan keeps its owner thread");
    r.detach(COMPOSITOR, FB);
    assert_eq!(r.given_back, vec![(START, LEN)], "given back once, on the detach");
}

#[test]
fn the_main_threads_orphan_loses_the_workers_pages() {
    let mut r = two_threads();
    let mut free = grant_frames();
    free.push(0x80_0000);
    r.orphans.push(PageOrphan { owner: GPU, handle: FB, frames: free.clone(), free });
    assert!(r.release(WORKER, Vec::new()));
    assert_eq!(r.orphans[0].free, vec![0x80_0000]);
}

#[test]
fn the_owners_own_attach_is_no_holder_whichever_thread_maps() {
    let mut r =
        Registry { attaches: vec![(FB, GPU)], threads: vec![(WORKER, GPU)], ..Default::default() };
    assert!(!r.release(WORKER, vec![(FB, GPU, grant_frames())]));
    assert_eq!(r.given_back, vec![(START, LEN)]);
}

#[test]
fn another_process_is_never_matched() {
    let mut r = Registry { attaches: vec![(FB, COMPOSITOR)], ..Default::default() };
    r.orphans.push(PageOrphan {
        owner: GPU,
        handle: FB,
        frames: grant_frames(),
        free: grant_frames(),
    });
    assert!(!r.release(WORKER, Vec::new()), "pid 8 is not a thread of pid 7 here");
    assert_eq!(r.orphans[0].free.len(), 4, "pid 7's orphan untouched");
}
