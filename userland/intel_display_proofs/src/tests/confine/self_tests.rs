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
//! The watch itself: a stray, a firmware entry written over, and an entry
//! left on a frame given back are each caught.

use super::kit::{arm, unheld, FIRMWARE};
use crate::engine::ggtt::{map, unmap};
use crate::engine::ggtt_pte::Pte;
use crate::model::Gen9;

const TOTAL: u32 = 1 << 12;

#[test]
fn a_page_not_held_written_present_is_a_stray() {
    let mut m = Gen9::new(TOTAL as usize);
    map(&mut m, Pte::Wide, 0, &[FIRMWARE.0; 4], TOTAL).unwrap();
    arm(&mut m, &[0x1000_0000], 2);
    map(&mut m, Pte::Wide, 8, &[0x1000_0000, 0x1000_1000], TOTAL).unwrap();
    assert!(m.confine.strays.is_empty(), "held pages");
    map(&mut m, Pte::Wide, 10, &[0x1000_2000], TOTAL).unwrap();
    assert_eq!(m.confine.strays.len(), 1, "one page past the frame");
    map(&mut m, Pte::Wide, 0, &[FIRMWARE.0], TOTAL).unwrap();
    assert_eq!(m.confine.strays.len(), 2, "firmware's entry written over");
    unmap(&mut m, Pte::Wide, 0, 1, TOTAL).unwrap();
    assert_eq!(m.confine.strays.len(), 2, "cleared is no stray");
    assert_eq!(unheld(&m).len(), 1, "and the stray still in the table");
}

#[test]
fn an_entry_left_on_a_frame_given_back_is_stale() {
    let mut m = Gen9::new(TOTAL as usize);
    arm(&mut m, &[0x1000_0000, 0x2000_0000], 1);
    map(&mut m, Pte::Wide, 3, &[0x1000_0000, 0x2000_0000], TOTAL).unwrap();
    unmap(&mut m, Pte::Wide, 3, 1, TOTAL).unwrap();
    m.free([(0x1000_0000, 0x1000_1000)]);
    assert!(m.confine.stale.is_empty(), "unmapped first");
    m.free([(0x2000_0000, 0x2000_1000)]);
    assert_eq!(m.confine.stale, vec![(4, m.ggtt[4])], "still mapped when given back");
    assert_eq!(unheld(&m).len(), 1);
}
