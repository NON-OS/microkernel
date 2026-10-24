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

//! What the driver says of itself (CAPS, MODES), what STATE does after a
//! guest reset, the rectangles a copy asks the host to redraw, and a ring
//! whose indices are out of range.

use super::util::{ask, primed, up, words};
use crate::bench::Bench;
use crate::model::with_svga;
use nonos_gfx_serve::Shadow;
use nonos_gfx_wire::body::caps::{MODE_RANGE, MODE_SET};
use nonos_gfx_wire::body::mode::{CURRENT, PREFERRED};
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::errno::ETIMEDOUT;
use nonos_gfx_wire::{kind, ops, Caps, Modes, Rect, State};

#[test]
fn caps_names_svga_ii_its_range_and_what_vram_holds() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |s| s.max = (2560, 1600));
    let c = Caps::decode(&ask(&mut d.server, ops::CAPS, &[]).1).unwrap();
    assert_eq!((c.kind, c.outputs, c.flags), (kind::SVGA_II, 1, MODE_SET | MODE_RANGE));
    assert_eq!((c.max_width, c.max_height, c.surface_budget), (2560, 1600, 16 << 20));
}

#[test]
fn modes_lists_the_current_mode_first_then_only_what_fits() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let m = Modes::decode(&ask(&mut d.server, ops::MODES, &[]).1).unwrap();
    let sizes: Vec<_> = m.list[..m.len].iter().map(|m| (m.width, m.height)).collect();
    assert_eq!(sizes, [(1024, 768), (800, 600), (640, 480)]);
    assert_eq!(m.list[0].flags & (CURRENT | PREFERRED), CURRENT | PREFERRED);
    let mut small = Bench::new(64 << 10, 2 << 20);
    let mut d = up(&mut small, |_| {});
    let m = Modes::decode(&ask(&mut d.server, ops::MODES, &[]).1).unwrap();
    let sizes: Vec<_> = m.list[..m.len].iter().map(|m| (m.width, m.height)).collect();
    assert_eq!(sizes, [(800, 600), (640, 480)], "1024x768 is not offered in 2 MiB");
}

#[test]
fn state_after_a_guest_reset_sets_the_mode_again_and_says_lost() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let st = State::decode(&ask(&mut d.server, ops::STATE, &[]).1).unwrap();
    assert_eq!(st.flags & (LOST | SCANNING_OUT), SCANNING_OUT, "nothing lost yet");
    with_svga(|s| (s.enable, s.width, s.height, s.bpp) = (0, 640, 480, 8));
    let lost = State::decode(&ask(&mut d.server, ops::STATE, &[]).1).unwrap();
    assert_eq!(lost.flags & (LOST | SCANNING_OUT), LOST | SCANNING_OUT);
    assert_eq!(lost.generation, st.generation + 1, "the compositor repaints all of it");
    assert_eq!(with_svga(|s| (s.enable, s.width, s.height, s.bpp)), (1, 1024, 768, 32));
}

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
