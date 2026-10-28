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
//! A frame held to the capsule heap (16 MiB, the libc's default): the
//! largest bound accepted, kept twice (the frame being gathered and the
//! last whole one), leaves half the heap, so a 4K camera is refused at
//! negotiation instead of aborting the driver, and the assembler never
//! holds more than those two buffers however its payloads arrive.

use nonos_uvc::{camera, frame_bound, frame_cap, Assembler, Probe, Refused};

use crate::webcam::config;

const HEAP: u64 = 16 << 20;

/// The YUY2 640x480 frame resized to `w` x `h`, bound two bytes a pixel.
fn bound(w: u16, h: u16) -> Result<u32, Refused> {
    let mut c = camera(&config()).unwrap();
    let fr = &mut c.formats[0].frames[0];
    (fr.width, fr.height, fr.max_bytes) = (w, h, u32::from(w) * u32::from(h) * 2);
    frame_bound(&c, &Probe { format: 1, frame: 1, ..Probe::default() }, frame_cap(HEAP))
}

#[test]
fn both_copies_of_the_largest_frame_leave_half_the_heap() {
    assert!(2 * frame_cap(HEAP) <= HEAP / 2);
    assert_eq!(bound(1920, 1080), Ok(1920 * 1080 * 2), "1080p YUY2 fits");
    assert_eq!(bound(3840, 2160), Err(Refused::TooLarge), "4K is refused");
}

#[test]
fn the_assembler_holds_two_bounds_at_most() {
    let max = 1000;
    let mut a = Assembler::new(max);
    let mut fid = 0u8;
    for _ in 0..8 {
        for last in [false, false, true] {
            let mut p = vec![2, 0x80 | fid | if last { 0x2 } else { 0 }];
            p.extend([7u8; 300]);
            a.push(&p);
            assert!(a.held() <= 2 * max, "held {} past twice {max}", a.held());
        }
        assert_eq!(a.frame().len(), 900);
        fid ^= 1;
    }
}
