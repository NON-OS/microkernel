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

//! The compositor may read the shell's surface at any moment of a paint. A
//! chrome paint clears to transparent and draws the dock back; while it does,
//! the surface must still hold the whole previous frame, and once it returns
//! the whole new one.

use crate::whole_frame::draw_whole;

const W: usize = 16;
const H: usize = 8;
const TRANSPARENT: u32 = 0;
const DOCK: u32 = 0xFF20_2428;
const DOCK_LIT: u32 = 0xFF30_3840;

#[test]
fn the_surface_is_untouched_until_the_frame_is_whole() {
    let mut surface = vec![DOCK; W * H];
    let target = surface.as_mut_ptr() as u64;
    let mut back = Vec::new();
    let mut during = Vec::new();
    draw_whole(&mut back, target, W * H, W, |va| {
        let px = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, W * H) };
        px.fill(TRANSPARENT);
        // The compositor reads the surface now, between the clear and the
        // dock being drawn again.
        during = unsafe { core::slice::from_raw_parts(target as *const u32, W * H) }.to_vec();
        for p in px[(H - 2) * W..].iter_mut() {
            *p = DOCK_LIT;
        }
    });
    assert!(
        during.iter().all(|&p| p == DOCK),
        "a composite during the paint read a cleared surface: the dock blinked"
    );
    assert!(surface[(H - 2) * W..].iter().all(|&p| p == DOCK_LIT));
    assert!(surface[..(H - 2) * W].iter().all(|&p| p == TRANSPARENT));
}

#[test]
fn only_changed_rows_are_written_and_the_frame_is_reused() {
    let mut surface = vec![DOCK; W * H];
    let target = surface.as_mut_ptr() as u64;
    let mut back = Vec::new();
    draw_whole(&mut back, target, W * H, W, |va| {
        let px = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, W * H) };
        px.fill(DOCK);
    });
    let kept = back.as_ptr();
    // A clock tick: only the top row changes.
    draw_whole(&mut back, target, W * H, W, |va| {
        let px = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, W * H) };
        px.fill(DOCK);
        px[3] = DOCK_LIT;
    });
    assert_eq!(back.as_ptr(), kept, "the frame is not allocated again");
    assert_eq!(surface[3], DOCK_LIT);
    assert!(surface[W..].iter().all(|&p| p == DOCK));
}
