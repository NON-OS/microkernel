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

//! The framebuffer is cleared only once the surface exists, so an attempt
//! refused at the surface, for its memory or its entry, leaves the
//! firmware's picture showing: the last GOP desktop a compositor drew
//! before the claim stopped the kernel's blit. The driver used to clear it
//! before making the surface.

use nonos_libc::{lend, refuse_surfaces};

use super::left::{showing, state};
use crate::adapter::{adapter, found, REG_LEN};
use crate::error::BgaError;
use crate::setup::run;

/// What the GOP desktop left in each pixel.
fn picture(i: usize) -> u32 {
    0x00C0_FFEE ^ i as u32
}

#[test]
fn an_attempt_refused_at_the_surface_leaves_the_firmwares_picture_showing() {
    let (w, h, pixels) = (64, 48, 64 * 48);
    let refusals: [(&str, fn()); 2] =
        [("entry", || refuse_surfaces(true)), ("memory", || lend(0, 0))];
    for (refused, refuse) in refusals {
        let a = adapter(REG_LEN, pixels * 4);
        showing(&a, w, h);
        (0..pixels).for_each(|i| a.fb.present32(i * 4, picture(i)));
        let firmware = state(&a);
        refuse();
        assert_eq!(run(found(pixels * 4)).err(), Some(BgaError::SurfaceRefused), "{refused}");
        let shown = (0..pixels).all(|i| a.fb.wrote32(i * 4) == picture(i));
        assert!(shown, "the picture was cleared before the surface's {refused} was refused");
        assert_eq!(state(&a), firmware, "{refused}: the firmware's mode was not left as it was");
        refuse_surfaces(false);
    }
}
