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

//! The 2048 window holds the whole board: the content area the runner gives
//! it, after the fit rule and the frame, is the layout the game paints.

use nonos_app_skeleton::runner::chrome::quarters_for;
use nonos_app_skeleton::runner::fit_rule::fit_manifest;
use nonos_toolkit::decorations::content_rect_at;

#[path = "../../../capsule_game_2048/src/game/manifest.rs"]
mod manifest;

#[test]
fn the_content_area_is_the_layout_on_every_mode() {
    for (w, h) in [(1920, 1080), (2560, 1440), (1280, 800), (1024, 768)] {
        let at = fit_manifest(manifest::manifest(), w, h);
        let c = content_rect_at(at.width, at.height, false, quarters_for(w, h));
        let size = (c.w, c.h);
        assert!(c.w >= manifest::WIDTH && c.h >= manifest::HEIGHT, "{w}x{h}: content {size:?}");
    }
}
