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

//! A driver holding the boot framebuffer (a GOP present refused, EBUSY):
//! the list starts again at once, and a refusal on every frame still walks
//! it to the driver that holds it.

use crate::display::backoff::{Backoff, FIRST_MS};

#[test]
fn bootfb_holding_the_framebuffer_starts_the_list_again_at_once() {
    let mut b = Backoff::new(0);
    for t in [500, 1000, 3000, 5000, 9000] {
        b.due(t, 5);
    }
    b.held(9100);
    assert_eq!((b.due(9100, 5), b.gap_ms), (Some(0), FIRST_MS));
}

#[test]
fn a_refusal_every_frame_still_walks_the_list_to_the_driver_holding_it() {
    /*
     * While a driver holds the framebuffer the kernel refuses every GOP
     * frame, and each refusal reaches held(). vmware-svga is the second
     * name; restarted on each refusal, the first name was asked for ever.
     */
    let mut b = Backoff::new(0);
    let mut asked = Vec::new();
    for t in (100..=3000).step_by(16) {
        b.held(t);
        if let Some(i) = b.due(t, 4) {
            asked.push(i);
        }
    }
    assert_eq!(&asked[..4], [0, 1, 2, 3], "one round, every name, in order");
    assert_eq!(asked[4], 0, "and the next refusal after the round starts another");
}
