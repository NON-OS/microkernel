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

//! The run's owner releases its device, or unmaps the grant, while alive.

use super::common::{grant_frames, slot, Registry, COMPOSITOR, FB, GPU, RUN};

fn scanout(attaches: Vec<(u32, u32)>) -> Registry {
    Registry {
        slots: vec![slot(FB, GPU, grant_frames(), Some(RUN))],
        attaches,
        ..Default::default()
    }
}

#[test]
fn an_attached_surface_keeps_its_grant_until_the_last_detach() {
    let mut r = scanout(vec![(FB, GPU), (FB, COMPOSITOR)]);
    assert!(r.release(RUN), "held for the compositor");
    assert!(r.given_back.is_empty(), "nothing freed under the compositor's mapping");
    assert_eq!(r.orphans.len(), 1, "the holder's detach still finds the frames");
    assert_eq!(r.orphans[0].frames, grant_frames());
    assert!(r.orphans[0].free.is_empty(), "no page of a grant is freed one by one");
    r.drop_attach(COMPOSITOR, FB);
    assert_eq!(r.given_back, vec![RUN], "given back on the last detach");
    assert!(r.pages_freed.is_empty(), "and through the broker alone");
    r.drop_attach(COMPOSITOR, FB);
    assert_eq!(r.given_back.len(), 1, "and only once");
}

#[test]
fn with_no_other_holder_the_grant_is_freed_at_once() {
    let mut r = scanout(vec![(FB, GPU)]);
    assert!(!r.release(RUN), "the owner's own attach holds nothing");
    assert_eq!(r.given_back, vec![RUN]);
    assert!(r.orphans.is_empty() && r.runs.is_empty() && r.slots.is_empty());
    let mut r = Registry::default();
    assert!(!r.release(RUN), "a grant under no surface");
}

#[test]
fn a_grant_under_two_surfaces_goes_back_once_after_both() {
    let (a, b, frames) = (0x21, 0x22, grant_frames());
    let slots = vec![
        slot(a, GPU, frames[..2].to_vec(), Some(RUN)),
        slot(b, GPU, frames[2..].to_vec(), Some(RUN)),
    ];
    let mut r = Registry { slots, attaches: vec![(a, COMPOSITOR), (b, 9)], ..Default::default() };
    assert!(r.release(RUN));
    assert_eq!(r.runs, vec![RUN], "one record for the run");
    r.drop_attach(COMPOSITOR, a);
    assert!(r.given_back.is_empty(), "the second surface is still mapped");
    r.drop_attach(9, b);
    assert_eq!(r.given_back, vec![RUN]);
}
