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

//! The chain's two turns outside draw and present: adopting the buffer a
//! layer was first submitted with, and giving back a drawn buffer whose
//! present the compositor refused.

use crate::swapchain::swapchain::{Chain, Misuse, Slot};
use crate::swapchain::swapchain_damage::{Damage, Stale};

const ALL: Damage = Damage { x: 0, y: 0, width: 8, height: 8 };

#[test]
fn the_adopted_buffer_is_shown_and_never_handed_out_first() {
    let mut c = Chain::new(2).expect("two");
    c.adopt(0).expect("nothing shown yet");
    assert_eq!((c.slot(0), c.shown()), (Some(Slot::Shown), Some(0)));
    let (i, stale) = c.acquire().expect("the other");
    assert_eq!((i, stale), (1, Stale::Whole));
    assert_eq!(c.acquire(), None, "the shown one is not free");
    c.present(1, ALL, |_| 4).expect("drawing");
    assert_eq!(c.slot(0), Some(Slot::Retiring { fence_was: 4 }));
    c.fence(0, 5);
    assert_eq!(c.acquire(), Some((0, Stale::Box(ALL))));
}

#[test]
fn adopting_after_the_chain_started_is_refused() {
    let mut c = Chain::new(2).expect("two");
    let _ = c.acquire().expect("free");
    assert_eq!(c.adopt(1), Err(Misuse::AlreadyStarted));
    assert_eq!(Chain::new(2).expect("two").adopt(2), Err(Misuse::NoSuchBuffer));
}

#[test]
fn a_refused_present_leaves_the_shown_buffer_shown() {
    let mut c = Chain::new(2).expect("two");
    c.adopt(0).expect("nothing shown yet");
    let (i, _) = c.acquire().expect("free");
    c.cancel(i).expect("was drawing");
    assert_eq!((c.shown(), c.slot(i)), (Some(0), Some(Slot::Free)));
    // What it holds was never shown: all of it is out of date.
    assert_eq!(c.acquire(), Some((i, Stale::Whole)));
    assert_eq!(c.cancel(0), Err(Misuse::NotDrawing), "the shown one");
    assert_eq!(c.cancel(5), Err(Misuse::NoSuchBuffer));
}
