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

//! The bar a window opens below is the bar the shell draws. The shell scales
//! its 46 pixel bar by the brand rule; the window manager kept 46 everywhere,
//! so on a 1920 by 1080 canvas a new window opened twelve pixels under the
//! bar it should sit below.

use super::constants::menubar_for;

/* The shell's `px(46)` at the brand rule's quarters for each canvas. */
#[test]
fn the_bar_is_the_one_the_shell_draws() {
    assert_eq!(menubar_for(1280, 800), 46);
    assert_eq!(menubar_for(1920, 1080), 58);
    assert_eq!(menubar_for(1280, 1024), 58);
    assert_eq!(menubar_for(2560, 1440), 69);
    assert_eq!(menubar_for(3840, 2160), 92);
}

#[test]
fn a_portrait_canvas_goes_by_its_short_side() {
    assert_eq!(menubar_for(1080, 1920), 58);
}

/* The dock's band, the shell's px(64) dock and px(16) gap, at each scale. */
#[test]
fn the_dock_band_is_the_one_the_shell_draws() {
    use super::constants::dock_for;
    assert_eq!(dock_for(1280, 800), 80);
    assert_eq!(dock_for(1920, 1080), 100);
    assert_eq!(dock_for(2560, 1440), 120);
    assert_eq!(dock_for(3840, 2160), 160);
}
