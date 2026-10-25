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

//! The owner exits: keep_at_exit orphans the surface first, then the DMA
//! release finds the orphan.

use super::common::{grant_frames, Registry, COMPOSITOR, FB, GPU, LEN, START};
use super::dma_rule::{overlaps, strip, PageOrphan};

/// exit_owned::orphan_owned for a surface over the grant and one heap page.
fn orphaned_at_exit(r: &mut Registry) {
    let mut frames = grant_frames();
    frames.push(0x80_0000);
    let mut free = frames.clone();
    strip(&mut free, |f| *f, START, LEN);
    assert_eq!(free, vec![0x80_0000], "only the heap page is freed per page");
    r.orphans.push(PageOrphan { owner: GPU, handle: FB, frames, free });
}

#[test]
fn an_exit_holds_the_grant_for_the_compositor() {
    let mut r = Registry { attaches: vec![(FB, COMPOSITOR)], ..Default::default() };
    orphaned_at_exit(&mut r);
    assert!(r.release(GPU, Vec::new()), "the orphan's holder keeps the grant");
    assert!(r.given_back.is_empty());
    r.detach(COMPOSITOR, FB);
    assert_eq!(r.given_back, vec![(START, LEN)], "freed once, on the detach");
}

#[test]
fn the_release_takes_its_pages_off_any_orphans_free_list() {
    // An orphan made before K15, or by a thread's exit, lists every page.
    let mut r = Registry { attaches: vec![(FB, COMPOSITOR)], ..Default::default() };
    let mut free = grant_frames();
    free.push(0x80_0000);
    r.orphans.push(PageOrphan { owner: GPU, handle: FB, frames: free.clone(), free });
    assert!(r.release(GPU, Vec::new()));
    assert_eq!(r.orphans[0].free, vec![0x80_0000], "the grant's pages are the broker's");
}

#[test]
fn a_holder_gone_before_the_release_lets_it_free_at_once() {
    let mut r = Registry::default();
    orphaned_at_exit(&mut r);
    assert!(!r.release(GPU, Vec::new()));
    assert_eq!(r.given_back, vec![(START, LEN)]);
    assert_eq!(r.orphans[0].free, vec![0x80_0000], "never a grant page per page");
}

#[test]
fn a_grant_is_a_half_open_range() {
    assert!(overlaps([START], START, LEN) && overlaps([START + LEN - 1], START, LEN));
    assert!(!overlaps([START + LEN], START, LEN) && !overlaps([START - 1], START, LEN));
    assert!(!overlaps([u64::MAX], u64::MAX - 1, 4), "an end past the top saturates");
}
