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

//! FLUSH on the taken-over engine: the rectangles reach the screen at the
//! next flip, both surfaces keep the whole picture, and a rectangle that
//! does not fit changes nothing.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::Rect;

use super::wire_rig::{rig, shadow_bytes};
use crate::engine::pipe::Pipe;

fn shadow(bytes: &[u8]) -> Shadow<'_> {
    Shadow { bytes, width: 1920, height: 1080, stride: 7680, cursor_offset: 0 }
}

#[test]
fn a_flush_is_on_screen_when_it_returns() {
    let (mut d, model) = rig();
    let px = shadow_bytes(0xFF12_3456);
    let r = Rect { x: 100, y: 200, width: 30, height: 40 };
    d.flush(0, &[r], &shadow(&px)).unwrap();
    let m = model.borrow();
    assert_eq!(m.pixel(Pipe::A, 100, 200), Some(0xFF12_3456));
    assert_eq!(m.pixel(Pipe::A, 129, 239), Some(0xFF12_3456));
    assert_eq!(m.pixel(Pipe::A, 130, 200), Some(0xFF00_00BB), "only the rectangle");
    assert_eq!(d.scan.front, 1, "flipped to the surface that was drawn");
    assert_eq!(d.frames.fences, 1, "fenced before the flip");
}

#[test]
fn both_surfaces_keep_every_flush() {
    /* A second FLUSH draws into the surface the first flipped away from;
     * unless the first's rectangle was copied there too, that flip would
     * bring back the old picture where it was. */
    let (mut d, model) = rig();
    let (one, two) = (shadow_bytes(0xFF00_0011), shadow_bytes(0xFF00_0022));
    d.flush(0, &[Rect { x: 0, y: 0, width: 8, height: 8 }], &shadow(&one)).unwrap();
    d.flush(0, &[Rect { x: 500, y: 500, width: 8, height: 8 }], &shadow(&two)).unwrap();
    let m = model.borrow();
    assert_eq!(d.scan.front, 0);
    assert_eq!(m.pixel(Pipe::A, 3, 3), Some(0xFF00_0011), "the first flush survives");
    assert_eq!(m.pixel(Pipe::A, 503, 503), Some(0xFF00_0022));
}

#[test]
fn a_rectangle_outside_the_output_changes_nothing() {
    let (mut d, model) = rig();
    let px = shadow_bytes(0xFF00_0033);
    let rects =
        [Rect { x: 0, y: 0, width: 4, height: 4 }, Rect { x: 1919, y: 0, width: 2, height: 1 }];
    assert_eq!(d.flush(0, &rects, &shadow(&px)), Err(EINVAL));
    assert_eq!(d.scan.front, 0, "no flip");
    assert_eq!(model.borrow().pixel(Pipe::A, 1, 1), Some(0xFF00_00AA));
    let short = vec![0u8; 16];
    let tiny = Shadow { bytes: &short, width: 2, height: 2, stride: 8, cursor_offset: 0 };
    let r = Rect { x: 0, y: 0, width: 4, height: 4 };
    assert_eq!(d.flush(0, &[r], &tiny), Err(EINVAL), "past the shadow");
}
