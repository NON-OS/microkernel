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

//! A frame handed to a wire driver in batches: one FLUSH for the frame's
//! rectangles, never one per rectangle (a flipping driver waits a vertical
//! blank per FLUSH), and a refused FLUSH's rectangles all shown later.

use super::batch_rig::*;
use crate::drain_batch::BATCH;
use nonos_gfx_wire::body::flush::MAX_RECTS;

#[test]
fn a_frame_goes_in_one_flush() {
    let mut d = display(&[(0, 0), (100, 0), (0, 100), (100, 100)]);
    frame(&mut d, |_| true).unwrap();
    assert_eq!(d.flushes.len(), 1);
    assert_eq!(d.flushes[0].len(), 4);
    assert_eq!(BATCH, MAX_RECTS, "as many as the wire takes");
}

#[test]
fn nothing_visible_sends_nothing() {
    let mut d = display(&[(0, 0), (100, 100)]);
    frame(&mut d, |r| r.x == 100).unwrap();
    assert_eq!(d.flushes, vec![vec![(100, 100)]]);
    let mut d = display(&[(0, 0)]);
    frame(&mut d, |_| false).unwrap();
    assert!(d.flushes.is_empty(), "no FLUSH with no rectangle");
}

#[test]
fn a_refused_flush_is_shown_whole_by_the_next_frame() {
    let mut d = display(&[(0, 0), (100, 0), (0, 100)]);
    d.refuse = 1;
    assert!(frame(&mut d, |_| true).is_err());
    assert!(d.flushes.is_empty());
    frame(&mut d, |_| true).unwrap();
    let mut shown = d.flushes.concat();
    shown.sort_unstable();
    assert_eq!(shown, vec![(0, 0), (0, 100), (100, 0)]);
}
