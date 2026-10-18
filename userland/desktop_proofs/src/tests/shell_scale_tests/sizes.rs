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

//! Sizes and type grow together with the scale.

use super::latch::SCALE_LATCH;
use crate::ui_font::{px, scale, scaled, set_scale};

#[test]
fn sizes_and_type_grow_together() {
    let _held = SCALE_LATCH.lock().unwrap_or_else(|e| e.into_inner());
    set_scale(4);
    assert_eq!((px(46), px(7), scale()), (46, 7, 1));
    assert_eq!(scaled(13.0), 13.0);

    set_scale(5);
    // The menu bar, a dock slot and its gap, at 1.25, to the nearest pixel.
    assert_eq!((px(46), px(7), px(1)), (58, 9, 1));
    assert_eq!(scaled(13.0), 16.25);
    // Glyphs drawn in whole pixels stay at one until the scale is two.
    assert_eq!(scale(), 1);

    set_scale(6);
    assert_eq!((px(46), scale()), (69, 1));
    assert_eq!(scaled(12.0), 18.0);

    set_scale(8);
    assert_eq!((px(46), scale()), (92, 2));
    assert_eq!(scaled(13.5), 27.0);

    // Below one to one is never asked for; a zero is read as one.
    set_scale(0);
    assert_eq!((px(46), scale()), (46, 1));
    set_scale(4);
}
