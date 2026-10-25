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

//! The owner's silence on the boot framebuffer: the picture the
//! compositor last flushed is gone from the framebuffer, not left there.

use super::serve::{ask, run, words};
use crate::fb::Layout;
use nonos_gfx_serve::OWNER_SILENCE_MS;
use nonos_gfx_wire::ops;

#[test]
fn a_silent_owner_s_picture_is_wiped_from_the_framebuffer() {
    let mut mem = vec![0u32; 20 * 8];
    let (mut s, sf) = run(Layout::BlueFirst, &mut mem);
    s.heard(7, 0);
    let base = s.slots[0].current.unwrap().region.base;
    s.platform.memory.get_mut(&base).unwrap().fill(0x33);
    assert_eq!(ask(&mut s, ops::FLUSH, &words(&[sf.serial, 1, 0, 0, 16, 8])).0, 0);
    assert!(mem.chunks(20).all(|r| r[..16].iter().all(|&p| p & 0xFF_FFFF == 0x33_3333)), "shown");
    assert!(s.watch(OWNER_SILENCE_MS));
    assert!(mem.chunks(20).all(|r| r[..16].iter().all(|&p| p & 0xFF_FFFF == 0)), "wiped");
    assert!(s.platform.memory[&base].iter().all(|&b| b == 0), "the primary scrubbed");
}
