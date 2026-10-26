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

//! K15b's threads under L35: a run is matched by the run its surface was
//! registered over, whichever thread tears it down; an exit strips the
//! grants of its own process alone; holders count against the owner.

use super::common::START;
use super::common::{grant_frames, slot, Orphan, Registry, COMPOSITOR, FB, GPU, HEAP, LEN, RUN};

/// The GPU capsule's worker thread, which maps the framebuffer grant.
const WORKER: u32 = 8;

#[test]
fn a_workers_grant_under_the_main_threads_surface_is_held() {
    let slots = vec![slot(FB, GPU, grant_frames(), Some(RUN))];
    let mut r = Registry { slots, attaches: vec![(FB, COMPOSITOR)], ..Default::default() };
    assert!(r.release(RUN), "found by its run, the worker's");
    assert_eq!(r.orphans[0].owner, GPU, "the orphan keeps its owner thread");
    r.drop_attach(COMPOSITOR, FB);
    assert_eq!(r.given_back, vec![RUN], "given back once, on the detach");
}

#[test]
fn the_main_threads_orphan_loses_the_workers_pages_and_no_other_process() {
    let frames = || [grant_frames(), vec![HEAP]].concat();
    for (holder, want) in [(WORKER, vec![HEAP]), (12, frames())] {
        let slots = vec![slot(FB, GPU, frames(), None)];
        let mut r = Registry { slots, attaches: vec![(FB, COMPOSITOR)], ..Default::default() };
        r.exit(GPU, &[(holder, START, LEN)], |h| h == GPU || h == WORKER);
        assert_eq!(r.orphans[0].free, want, "grant of pid {holder}");
    }
}

#[test]
fn another_process_is_never_matched() {
    let mut r = Registry { attaches: vec![(FB, COMPOSITOR)], ..Default::default() };
    let theirs =
        Orphan { owner: 12, handle: FB, frames: grant_frames(), free: grant_frames(), grant: None };
    r.orphans.push(theirs.clone());
    assert!(!r.release(RUN), "pid 12's surface is not this run's");
    assert_eq!(r.orphans, vec![theirs], "its free list untouched");
}

#[test]
fn the_owners_record_on_its_orphan_is_no_foreign_holder() {
    let mut r = Registry { attaches: vec![(FB, GPU)], ..Default::default() };
    let free = Vec::new();
    r.orphans.push(Orphan {
        owner: GPU,
        handle: FB,
        frames: grant_frames(),
        free,
        grant: Some(RUN),
    });
    // Still listed, it holds the run until the reclaim every let-go runs.
    assert!(r.release(RUN));
    r.free_orphans();
    r.free_unheld();
    assert!(r.orphans.is_empty(), "only its owner maps it");
    assert_eq!(r.given_back, vec![RUN], "given back once");
}
