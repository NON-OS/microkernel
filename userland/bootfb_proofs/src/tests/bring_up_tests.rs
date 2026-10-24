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

//! bootfb's bring-up on a model of exactly the kernel's record (lead L31):
//! announce, claim, mode, map, in that order; the first pixel where the
//! record's aux puts it; three-byte pixels refused with one line.

use super::firmware_model::Model;
use crate::bring_up::bring_up;
use crate::fb::Layout;
use crate::firmware::Why;
use crate::geometry::{BGR, BGRX, RGB, RGBX};
use nonos_gfx_serve::Shadow;
use nonos_gfx_wire::Rect;

#[test]
fn the_kernels_record_is_claimed_mapped_and_drawn_from_its_first_pixel() {
    let (mut m, r) = Model::new(64, 16, 512, BGRX, 0x40);
    let fb = bring_up(&mut m, &r).unwrap();
    assert_eq!(m.calls, ["announce", "claim", "geometry", "map"]);
    assert_eq!((fb.width, fb.height, fb.stride, fb.layout), (64, 16, 512, Layout::BlueFirst));
    assert_eq!((fb.len as u64, fb.physical_mm), (r.bar0.3 - 0x40, 300 | 170 << 16));
    let bytes = 0x1122_3344u32.to_le_bytes().repeat(64 * 16);
    let s = Shadow { bytes: &bytes, width: 64, height: 16, stride: 256, cursor_offset: 0 };
    fb.copy_in(&[Rect { x: 63, y: 15, width: 1, height: 1 }], &s);
    let last = 0x40 + 15 * 512 + 63 * 4;
    assert_eq!((m.byte(last), m.byte(last + 3)), (0x44, 0x11), "the last row's last pixel");
    assert_eq!(m.byte(last - 4), 0, "nothing else written");
    let (mut red, r) = Model::new(64, 16, 256, RGBX, 0);
    assert_eq!(bring_up(&mut red, &r).map(|f| f.layout), Ok(Layout::RedFirst));
}

#[test]
fn three_byte_pixels_are_refused_with_one_line_and_given_back() {
    for format in [RGB, BGR] {
        let (mut m, r) = Model::new(64, 16, 192, format, 0);
        assert_eq!(bring_up(&mut m, &r).err(), Some(Why::Format));
        assert_eq!(m.calls, ["announce", "claim", "geometry", "release"]);
        assert_eq!(m.said.len(), 1);
    }
}

#[test]
fn a_claim_refused_for_good_ends_with_nothing_held() {
    let (mut m, r) = Model::new(64, 16, 512, BGRX, 0x40);
    m.taken = true;
    assert_eq!(bring_up(&mut m, &r).err(), Some(Why::Refused));
    assert_eq!(m.calls, ["announce", "claim"], "nothing mapped, nothing to release");
}
