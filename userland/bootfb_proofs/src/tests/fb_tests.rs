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

//! The framebuffer checks and the one mode it offers.

use crate::display::BootFb;
use crate::fb::{Fb, Layout};
use nonos_gfx_serve::Display;
use nonos_gfx_wire::{kind, Mode};

#[test]
fn a_mapping_too_small_or_misaligned_is_no_framebuffer() {
    let mut mem = vec![0u32; 1024];
    let p = mem.as_mut_ptr() as *mut u8;
    assert!(Fb::new(p, 4096, (32, 32), 128, Layout::BlueFirst).is_some());
    assert!(Fb::new(p, 4095, (32, 32), 128, Layout::BlueFirst).is_none(), "one byte short");
    assert!(Fb::new(p, 4096, (32, 32), 124, Layout::BlueFirst).is_none(), "stride under a line");
    assert!(Fb::new(p, 4096, (0, 32), 128, Layout::BlueFirst).is_none());
    assert!(Fb::new(unsafe { p.add(1) }, 4000, (8, 8), 32, Layout::BlueFirst).is_none());
    assert!(Fb::new(p, usize::MAX, (u32::MAX, u32::MAX), u32::MAX, Layout::RedFirst).is_none());
}

#[test]
fn caps_modes_and_state_name_one_fixed_mode() {
    let mut mem = vec![0u32; 1024];
    let mut fb =
        Fb::new(mem.as_mut_ptr() as *mut u8, 4096, (32, 30), 128, Layout::BlueFirst).unwrap();
    fb.physical_mm = 300 | 200 << 16;
    let mut d = BootFb { fb, fence: || {} };
    let c = d.caps();
    assert_eq!(
        (c.kind, c.outputs, c.flags, c.max_width, c.max_height),
        (kind::BOOT_FB, 1, 0, 32, 30)
    );
    assert_eq!((c.surface_budget, c.physical_mm), (32 * 30 * 4, 300 | 200 << 16));
    let mut m = [Mode::default(); 20];
    assert_eq!(d.modes(0, &mut m), Ok(1));
    assert_eq!((m[0].width, m[0].height, m[0].flags), (32, 30, 1 | 2 | 4));
    let s = d.state(0).unwrap();
    assert_eq!((s.width, s.height, s.flags), (32, 30, 1));
}
