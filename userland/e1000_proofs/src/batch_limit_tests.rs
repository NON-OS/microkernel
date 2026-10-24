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

//! Where a batch stops: at its frame limit, leaving the rest for the next
//! one, and after one turn of the ring when every slot holds an error.

use crate::batch_ring::{body_frames, Ring};
use crate::constants::queue::RX_DESC_COUNT;

#[test]
fn a_batch_stops_at_its_frame_limit_and_the_rest_wait() {
    let mut r = Ring::new();
    for i in 0..5 {
        r.complete(i, &[i as u8; 64]);
    }
    let mut gave = Vec::new();
    assert_eq!(body_frames(&r.take(3, &mut gave).unwrap()).unwrap().len(), 3);
    assert_eq!(body_frames(&r.take(3, &mut gave).unwrap()).unwrap().len(), 2);
    assert_eq!(gave, [0, 1, 2, 3, 4]);
}

#[test]
fn a_ring_of_errors_is_looked_at_once_and_gives_no_batch() {
    let mut r = Ring::new();
    for i in 0..RX_DESC_COUNT as u16 {
        r.fail(i);
    }
    let mut gave = Vec::new();
    assert_eq!(r.take(24, &mut gave), None);
    assert_eq!(gave.len(), RX_DESC_COUNT);
}
