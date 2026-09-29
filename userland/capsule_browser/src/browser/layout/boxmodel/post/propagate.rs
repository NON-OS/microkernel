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

use super::super::build::Natural;
use super::super::layout_block::MAX_DEPTH;
use super::super::tree::{BoxKind, BoxNode};

/// What body hands up (CSS Backgrounds 3 section 2.11.2, Overflow 3
/// section 3.3). The root element's background paints the canvas; when the
/// root sets none, body's does instead and body paints none itself. The
/// root's overflow applies to the viewport, or body's while the root's is
/// visible; the viewport never clips, and the box it came from shows its
/// overflow.
pub(crate) fn propagate(root: &mut BoxNode, body_id: usize) {
    let root_id = root.dom_id;
    let root_bg = root.style.bg >> 24 != 0 || root.bg_image.is_some();
    let root_overflow = root.style.overflow_set();
    if root_overflow {
        root.style.give_overflow_to_viewport();
    }
    let body = root.children.iter_mut().find(|c| c.dom_id == body_id && body_id != root_id);
    let Some(body) = body else { return };
    if !root_bg {
        root.style.bg = core::mem::take(&mut body.style.bg);
        root.bg_image = body.bg_image.take();
        (root.style.bg_size, root.style.bg_repeat) = (body.style.bg_size, body.style.bg_repeat);
    }
    if !root_overflow {
        body.style.give_overflow_to_viewport();
    }
}

/// Give every image box its natural size from `natural`, keyed by its src.
pub(crate) fn fill_natural(n: &mut BoxNode, natural: Natural<'_>, depth: u32) {
    if depth > MAX_DEPTH {
        return;
    }
    if let BoxKind::Image { src, .. } = &n.kind {
        n.aux.natural = natural(src);
    }
    for c in &mut n.children {
        fill_natural(c, natural, depth + 1);
    }
}
