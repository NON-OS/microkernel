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

//! A bring-up refused once the device has its ring (here, a disk of no
//! sectors): the device is reset before the claim goes, and bus mastering
//! is switched off again first. The legacy rollback used to unmap the
//! queue, header and data grants with the device still pointed at them.

use std::cell::Cell;
use std::rc::Rc;

use nonos_libc::{events, on_event, Event};

use super::bench::bench;
use super::bringup_tests::masters;
use crate::setup;

#[test]
fn a_bring_up_refused_after_the_ring_was_handed_over_resets_before_the_claim_goes() {
    let dev = bench(8, 0, 1);
    dev.borrow_mut().msix = true;
    let at_release = Rc::new(Cell::new(None));
    let (watch, seen) = (dev.clone(), at_release.clone());
    on_event(move |e| {
        if e == Event::Release {
            seen.set(Some((watch.borrow().status, watch.borrow().pfn)));
        }
    });
    assert_eq!(setup::run().err(), Some("virtio-blk: zero capacity"));
    assert_eq!(at_release.get(), Some((0, 0)), "the device held its ring when the claim went");
    let log = events();
    let on = log.iter().rposition(|&e| masters(e)).expect("mastered before the capacity read");
    let off = log.iter().rposition(|&e| matches!(e, Event::Command(_)) && !masters(e));
    assert!(off.is_some_and(|off| on < off && off + 1 == log.len() - 1), "{log:?}");
}
