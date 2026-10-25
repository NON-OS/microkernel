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

//! The rectangles a copy asks the host to redraw, and a ring whose
//! indices are out of range.

use super::ask::{ask, primed, words};
use super::util::up;
use crate::bench::Bench;
use crate::model::with_svga;
use nonos_gfx_serve::Shadow;
use nonos_gfx_wire::errno::ETIMEDOUT;
use nonos_gfx_wire::{ops, Rect};

#[test]
fn a_copy_asks_the_host_to_redraw_only_rectangles_inside_the_mode() {
    let mut bench = Bench::standard();
    let d = up(&mut bench, |_| {});
    let shadow_bytes = vec![0u8; 1024 * 768 * 4];
    let shadow =
        Shadow { bytes: &shadow_bytes, width: 1024, height: 768, stride: 4096, cursor_offset: 0 };
    let r = |x, y, width, height| Rect { x, y, width, height };
    let rects =
        [r(0, 0, 8, 8), r(1020, 0, 8, 8), r(0, 767, 4, 2), r(5, 5, 0, 3), r(u32::MAX, 0, 2, 2)];
    assert_eq!(d.server.display.copy(&rects, &shadow), Ok(()));
    assert_eq!(with_svga(|s| std::mem::take(&mut s.updates)), [(0, 0, 8, 8)]);
}

#[test]
fn a_ring_with_its_indices_out_of_range_refuses_the_flush() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let sf = primed(&mut d.server, 1);
    bench.fifo[3] = 7;
    let st = ask(&mut d.server, ops::FLUSH, &words(&[sf.serial, 1, 0, 0, 8, 8])).0;
    assert_eq!(st, ETIMEDOUT, "STOP not word aligned: nothing is pushed");
    assert_eq!(bench.fifo[2], 4096, "NEXT_CMD unmoved");
}
