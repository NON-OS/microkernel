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

use super::damage::Rect;
use super::scene::SceneTable;
use super::scene_submit::rect_of;

// Raises `owner_pid`'s layer to the top of its band and returns the rectangle
// to repaint, or None when the draw order did not change.
//
// Only the raised layer's own rectangle can change: everywhere else the same
// layer is on top as before. The whole screen used to be recomposited on every
// focus change instead, a full frame (doubled onto the panel on a high density
// screen) between the press that raises a window and the first step of the
// drag that press starts.
pub fn raise_by_pid(scene: &mut SceneTable, owner_pid: u32) -> Option<Rect> {
    let mut union: Option<Rect> = None;
    scene.raise(owner_pid, |l| {
        let r = rect_of(l);
        union = Some(match union {
            None => r,
            Some(u) => {
                let x0 = u.x.min(r.x);
                let y0 = u.y.min(r.y);
                let x1 = (u.x + u.width).max(r.x + r.width);
                let y1 = (u.y + u.height).max(r.y + r.height);
                Rect { x: x0, y: y0, width: x1 - x0, height: y1 - y0 }
            }
        });
    });
    union
}
