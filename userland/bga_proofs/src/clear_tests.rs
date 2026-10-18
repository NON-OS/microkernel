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

//! Filling the framebuffer, and stopping where it ends.
//!
//! `clear` writes one 32-bit word per pixel of a `Span`, and a span exists
//! only for a mode whose pixels fit the mapping it was made from. A second
//! window here stands in for the framebuffer aperture, one pixel longer than
//! the mapping, so a test can watch the word after the last pixel.

use nonos_devmodel::FakeBar;

use crate::dispi::{clear, span};

const COLOR: u32 = 0x0010_2a3a;
const GUARD: u32 = 0xdead_beef;

/// An aperture of `pixels` pixels with one more pixel of guard behind it.
fn aperture(pixels: usize) -> FakeBar {
    let bar = FakeBar::new((pixels + 1) * 4);
    bar.present32(pixels * 4, GUARD);
    bar
}

/// Clear a `w` x `h` mode in an aperture mapped exactly that large.
fn cleared(w: u16, h: u16) -> FakeBar {
    let pixels = w as usize * h as usize;
    let fb = aperture(pixels);
    clear(&span(fb.base(), pixels as u64 * 4, w, h).expect("an exact fit"), COLOR);
    fb
}

#[test]
fn every_pixel_in_the_span_takes_the_colour_it_was_given() {
    /*
     * A clear that covers all but the last row is how a display comes up with
     * one band of whatever the firmware left behind, which reads as a corrupt
     * mode rather than as an arithmetic slip.
     */
    let fb = cleared(8, 8);
    for i in 0..64 {
        assert_eq!(fb.wrote32(i * 4), COLOR, "pixel {i} was not cleared");
    }
}

#[test]
fn the_pixel_past_the_end_of_the_span_is_left_alone() {
    /*
     * One pixel of overrun writes four bytes past the framebuffer mapping,
     * which on a booted machine faults the capsule or lands in whatever the
     * pager put next to the aperture. Neither points back at this loop.
     */
    assert_eq!(cleared(8, 8).wrote32(64 * 4), GUARD, "clear wrote past the last pixel");
}

#[test]
fn a_span_of_no_pixels_writes_nothing_at_all() {
    // A zero-sized mode must not touch the aperture, even one mapped empty.
    assert_eq!(cleared(0, 768).wrote32(0), GUARD, "an empty clear still wrote a pixel");
    assert_eq!(cleared(1024, 0).wrote32(0), GUARD, "an empty clear still wrote a pixel");
}
