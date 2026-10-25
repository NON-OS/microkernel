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

//! What the taken-over engine says of itself on the display wire, and the
//! colour order of firmware's picture.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::body::caps::{HW_CURSOR, MODE_RANGE, MODE_SET};
use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, PREFERRED};
use nonos_gfx_wire::errno::{ENOSYS, ENXIO};
use nonos_gfx_wire::{kind, Cursor, Mode};

use crate::display::{red_first, swap_red_blue};
use crate::engine::gen::PlaneKind;
use crate::tests::wire_rig::{h0, rig};

#[test]
fn caps_name_an_intel_engine_on_firmwares_mode_alone() {
    let (d, _) = rig();
    let c = d.caps();
    assert_eq!((c.kind, c.outputs), (kind::INTEL, 1));
    assert_eq!(c.flags & (MODE_SET | MODE_RANGE), 0, "takeover only: no mode setting on eDP");
    assert_eq!(c.flags & HW_CURSOR, HW_CURSOR, "the cursor plane");
    assert_eq!((c.max_width, c.max_height, c.width_align), (1920, 1080, 1));
    assert_eq!(c.surface_budget, 7680 * 1080, "one surface at its 64-byte stride");
}

#[test]
fn the_one_mode_is_firmwares_and_nothing_else_is_served() {
    let (mut d, _) = rig();
    let mut out = [Mode::default(); 4];
    assert_eq!(d.modes(0, &mut out), Ok(1));
    assert_eq!(
        out[0],
        Mode { width: 1920, height: 1080, flags: PREFERRED | CURRENT | FROM_DEVICE }
    );
    let bytes = vec![0u8; 4];
    let s = Shadow { bytes: &bytes, width: 1, height: 1, stride: 4, cursor_offset: 0 };
    assert_eq!(d.set_mode(0, 1280, 720, &s), Err(ENOSYS));
    assert_eq!(d.cursor(1, &Cursor::default(), &[]), Err(ENXIO), "no second output");
}

#[test]
fn a_red_first_plane_is_found_and_its_pixels_swapped() {
    let (d, _) = rig();
    let mut p = h0(&d).out.plane;
    assert!(!red_first(&p), "the model's firmware plane is blue first");
    p.ctl |= PlaneKind::Universal.order_bit();
    assert!(red_first(&p));
    p.kind = PlaneKind::Primary;
    p.ctl = 0xE << 26;
    assert!(red_first(&p), "DSPCNTR's RGBX");
    p.ctl = 0x6 << 26;
    assert!(!red_first(&p), "DSPCNTR's BGRX");
    let mut row = [1, 2, 3, 4, 5, 6, 7, 8];
    swap_red_blue(&mut row);
    assert_eq!(row, [3, 2, 1, 4, 7, 6, 5, 8]);
}
