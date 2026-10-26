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

//! The owner exits: keep_at_exit orphans its surfaces first (exit_owned.rs),
//! then the teardown of its grants finds the orphan holding the run.

use super::common::{grant_frames, slot, Registry, COMPOSITOR, FB, GPU, HEAP, LEN, RUN, START};
use super::dma_rule::strip;

const GPU_GRANTS: [(u32, u64, u64); 1] = [(GPU, START, LEN)];

fn exited(slot_grant: Option<(u64, u64)>, frames: Vec<u64>) -> Registry {
    let slots = vec![slot(FB, GPU, frames, slot_grant)];
    let mut r = Registry { slots, attaches: vec![(FB, COMPOSITOR)], ..Default::default() };
    r.exit(GPU, &GPU_GRANTS, |h| h == GPU);
    r
}

#[test]
fn an_exit_holds_the_grant_for_the_compositor() {
    let mut r = exited(Some(RUN), grant_frames());
    assert!(r.orphans[0].free.is_empty(), "a run's orphan lists nothing to free per page");
    assert!(r.release(RUN), "the orphan's holder keeps the run");
    assert!(r.given_back.is_empty());
    r.drop_attach(COMPOSITOR, FB);
    assert_eq!(r.given_back, vec![RUN], "freed once, on the detach");
    assert!(r.pages_freed.is_empty());
}

#[test]
fn an_exits_orphan_never_lists_a_page_of_its_groups_grants() {
    // A page-backed surface over the grant's pages and a heap page.
    let mut frames = grant_frames();
    frames.push(HEAP);
    let mut r = exited(None, frames);
    assert_eq!(r.orphans[0].free, vec![HEAP], "the grant's pages are the broker's");
    r.drop_attach(COMPOSITOR, FB);
    assert_eq!(r.pages_freed, vec![HEAP], "only the heap page is freed per page");
}

#[test]
fn a_holder_gone_before_the_release_lets_it_free_at_once() {
    let mut r = exited(Some(RUN), grant_frames());
    r.drop_attach(COMPOSITOR, FB);
    assert!(r.orphans.is_empty() && r.given_back.is_empty(), "no run was held yet");
    assert!(!r.release(RUN));
    assert_eq!(r.given_back, vec![RUN]);
    assert!(r.pages_freed.is_empty(), "never a grant page per page");
}

#[test]
fn a_grant_is_a_half_open_range() {
    let mut free = vec![START - 1, START, START + LEN - 1, START + LEN];
    strip(&mut free, |f| *f, START, LEN);
    assert_eq!(free, vec![START - 1, START + LEN]);
    let mut top = vec![u64::MAX - 1, u64::MAX];
    strip(&mut top, |f| *f, u64::MAX - 1, 4);
    assert_eq!(top, vec![u64::MAX], "an end past the top saturates");
}
