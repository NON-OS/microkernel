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

//! The owner releases its device, or unmaps the grant, while alive.

use super::common::{grant_frames, Registry, COMPOSITOR, FB, GPU, LEN, START};

#[test]
fn an_attached_surface_keeps_its_grant_until_the_last_detach() {
    let mut r = Registry { attaches: vec![(FB, GPU), (FB, COMPOSITOR)], ..Default::default() };
    assert!(r.release(GPU, vec![(FB, GPU, grant_frames())]), "held for the compositor");
    assert!(r.given_back.is_empty(), "nothing freed under the compositor's mapping");
    assert_eq!(r.orphans.len(), 1, "the holder's detach still finds the frames");
    assert!(r.orphans[0].free.is_empty(), "no page of a grant is freed one by one");
    r.detach(COMPOSITOR, FB);
    assert_eq!(r.given_back, vec![(START, LEN)], "given back on the last detach");
    r.detach(COMPOSITOR, FB);
    assert_eq!(r.given_back.len(), 1, "and only once");
}

#[test]
fn with_no_other_holder_the_grant_is_freed_at_once() {
    let mut r = Registry { attaches: vec![(FB, GPU)], ..Default::default() };
    assert!(
        !r.release(GPU, vec![(FB, GPU, grant_frames())]),
        "the owner's own attach holds nothing"
    );
    assert_eq!(r.given_back, vec![(START, LEN)]);
    assert!(r.orphans.is_empty() && r.held.is_empty());
    let mut r = Registry::default();
    assert!(!r.release(GPU, Vec::new()), "a grant under no surface");
}

#[test]
fn a_grant_under_two_surfaces_goes_back_once_after_both() {
    let (a, b) = (0x21, 0x22);
    let mut r = Registry { attaches: vec![(a, COMPOSITOR), (b, 9)], ..Default::default() };
    let frames = grant_frames();
    let live = vec![(a, GPU, frames[..2].to_vec()), (b, GPU, frames[2..].to_vec())];
    assert!(r.release(GPU, live));
    assert_eq!(r.held.len(), 1, "one record for the grant");
    r.detach(COMPOSITOR, a);
    assert!(r.given_back.is_empty(), "the second surface is still mapped");
    r.detach(9, b);
    assert_eq!(r.given_back, vec![(START, LEN)]);
}

#[test]
fn another_owners_orphan_is_left_alone() {
    let mut r = Registry { attaches: vec![(FB, COMPOSITOR)], ..Default::default() };
    let theirs = super::dma_rule::PageOrphan {
        owner: 12,
        handle: FB,
        frames: grant_frames(),
        free: grant_frames(),
    };
    r.orphans.push(theirs);
    assert!(!r.release(GPU, Vec::new()), "pid 12's surface is not this grant's");
    assert_eq!(r.orphans[0].free.len(), 4, "its free list untouched");
}
