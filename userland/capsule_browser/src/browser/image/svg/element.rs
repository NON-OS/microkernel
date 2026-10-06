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

use alloc::vec::Vec;

use super::attr::attr;
use super::path::parse_path;
use super::shapes::shape_polys;
use super::state::Paint;
use super::walk::{container, Walk};
use super::xml::Tag;

impl Walk<'_, '_> {
    /// Handle one opening tag under inherited state `cur`: open a group,
    /// paint a shape or path, or paint what a `use` names. False when the
    /// element and its subtree are not to be painted: a clip-path that
    /// does not resolve hides them.
    pub(super) fn element(
        &mut self,
        tag: &Tag,
        cur: Paint,
        stack: &mut Vec<(Paint, bool)>,
    ) -> bool {
        let mut p = cur.derive(tag.attrs, self.defs);
        let clip = attr(tag.attrs, "clip-path").filter(|v| v.trim() != "none");
        let polys = match tag.name {
            "path" => attr(tag.attrs, "d").map(parse_path),
            "rect" | "circle" | "ellipse" | "line" | "polyline" | "polygon" => {
                Some(shape_polys(tag.name, tag.attrs))
            }
            _ => None,
        };
        if let Some(polys) = polys {
            self.draw(&polys, &p, clip);
            return true;
        }
        if !container(tag.name) && tag.name != "use" {
            return true;
        }
        let owned = match clip {
            Some(v) => match self.group_clip(v, &mut p, cur) {
                Some(()) => true,
                None => return false,
            },
            None => false,
        };
        if tag.name == "use" {
            self.use_ref(tag.attrs, p);
            if owned {
                self.masks.pop();
            }
        } else if !tag.self_closing {
            stack.push((p, owned));
        } else if owned {
            self.masks.pop();
        }
        true
    }
}
