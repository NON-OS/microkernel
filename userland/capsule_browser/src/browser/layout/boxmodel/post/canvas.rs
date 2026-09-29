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

use super::super::display_list::{CanvasImage, DisplayList};
use super::super::tree::BoxNode;

/// The canvas background: the root box's color and image, which the box
/// tree build already took from body when the root set none. The root's
/// own fragment (the first laid) then paints neither, as the canvas under
/// the whole viewport shows them.
pub(crate) fn canvas(root: &BoxNode, frags: &mut DisplayList) -> (u32, Option<CanvasImage>) {
    let s = &root.style;
    if let Some(f) = frags.first_mut().filter(|f| f.node == root.dom_id) {
        f.bg = 0;
        f.bg_image = None;
    }
    let image =
        root.bg_image.clone().map(|url| CanvasImage { url, size: s.bg_size, repeat: s.bg_repeat });
    (s.bg, image)
}
