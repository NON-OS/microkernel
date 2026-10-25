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

//! The scale follows the canvas's short side.

use crate::shell_scale::quarters_for;

#[test]
fn the_scale_follows_the_canvas_short_side() {
    // (canvas, quarters): 1366x768 and the half canvas of a 2560x1440 panel
    // draw one to one; a 1080p canvas, or a 4K panel's half canvas, at 1.25.
    let cases = [
        ((1366, 768), 4),
        ((1280, 720), 4),
        ((1280, 800), 4),
        ((1920, 1080), 5),
        ((1920, 1200), 5),
        ((2560, 1600), 6),
        ((2880, 1620), 6),
        ((3840, 2160), 8),
    ];
    for ((w, h), q) in cases {
        assert_eq!(quarters_for(w, h), q, "{w}x{h}");
    }
}
