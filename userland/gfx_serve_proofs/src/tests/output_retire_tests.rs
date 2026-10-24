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

//! MODE_SET and RETIRE with several outputs: one primary waiting per
//! output, and a new compositor freeing every one of them.

use super::output_tests::{surface, two, Two};
use crate::model::wire::{ask, u32s, COMPOSITOR};
use nonos_gfx_wire::errno::EBUSY;
use nonos_gfx_wire::ops;

#[test]
fn mode_sets_wait_for_retire_per_output_and_retire_finds_any_serial() {
    let mut s = two();
    let (a, b) = (surface(&mut s, 0, COMPOSITOR), surface(&mut s, 1, COMPOSITOR));
    let set = |s: &mut Two, o, w, h| ask(s, COMPOSITOR, ops::MODE_SET, o, &u32s(&[w, h, 0])).0;
    assert_eq!(set(&mut s, 0, 1280, 720), 0);
    assert_eq!(set(&mut s, 1, 1024, 768), 0, "output 0's wait is not output 1's");
    assert_eq!(set(&mut s, 1, 800, 600), EBUSY);
    assert_eq!(s.display.heads[1].size, (1024, 768));
    assert_eq!(ask(&mut s, COMPOSITOR, ops::RETIRE, 0, &u32s(&[b.serial])).0, 0);
    assert!(s.slots[1].retiring.is_none() && s.slots[0].retiring.unwrap().serial == a.serial);
    assert_eq!(set(&mut s, 1, 800, 600), 0, "output 1 free again");
}

#[test]
fn a_new_compositor_frees_what_every_output_waited_to_retire() {
    let mut s = two();
    surface(&mut s, 0, COMPOSITOR);
    surface(&mut s, 1, COMPOSITOR);
    for o in 0..2 {
        ask(&mut s, COMPOSITOR, ops::MODE_SET, o, &u32s(&[1024, 768, 0]));
    }
    assert_eq!(s.platform.memory.len(), 4);
    surface(&mut s, 1, COMPOSITOR + 1);
    assert!(s.slots.iter().all(|x| x.retiring.is_none()));
    assert_eq!(s.platform.memory.len(), 2);
}
