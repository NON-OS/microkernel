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

//! Vertical band tiling and the default render geometry.

use crate::support::{lay, BOTH_RAILS, CHROME, NO_RAILS};

/// Every horizontal band has to abut the next one exactly: a gap paints
/// wallpaper through the chrome, an overlap paints one band over another.
#[test]
fn the_bands_tile_the_window_with_no_gap_or_overlap() {
    let l = lay(1440, 900, NO_RAILS);
    assert_eq!(l.body.y, CHROME.titlebar_h + CHROME.tabstrip_h + CHROME.body_pad_top);
    assert_eq!(l.input.y, l.body.y + l.body.h);
    assert_eq!(l.footer.y, l.input.y + l.input.h);
    assert_eq!(l.footer.y + l.footer.h, 900);
}

/// The regression guard for the shipping window geometry. `body.y` was 50 while
/// the content area still reserved 16px for the in-content tab strip; the strip
/// moved into the titlebar accessory, that band was reclaimed, and the body top
/// is now HEADER_H + BODY_PAD_TOP = 34. The guard is not weakened: `input.y` and
/// `footer.y` are anchored to the bottom of the window and must not have moved.
#[test]
fn the_default_render_pins_the_reclaimed_geometry() {
    let l = lay(520, 300, NO_RAILS);
    assert_eq!(l.body.y, 34, "BODY_TOP is HEADER_H + BODY_PAD_TOP");
    assert_eq!(l.input.y, 264, "input_y was 300 - (FOOTER_H + row_h)");
    assert_eq!(l.footer.y, 284, "body_max was 300 - FOOTER_H");
    assert_eq!(l.body.x + CHROME.text_left, 14, "TEXT_LEFT with no left rail");
}

/// The prompt sits directly under the scrollback, so it has to share the
/// body's column exactly or the caret drifts away from the text above it.
#[test]
fn the_input_shares_the_body_column() {
    let l = lay(1440, 900, BOTH_RAILS);
    assert_eq!(l.input.x, l.body.x);
    assert_eq!(l.input.w, l.body.w);
    assert_eq!(l.input.h, CHROME.row_h);
}
