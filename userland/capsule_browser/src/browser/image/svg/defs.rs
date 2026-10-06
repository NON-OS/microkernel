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
use super::gradient::{read_gradient, Gradient};
use super::xml::next_tag;

/// The most ids and gradients a document may define.
const MAX_IDS: usize = 4096;
const MAX_GRADIENTS: usize = 256;

/// What elements refer to by id: where each id'd element starts (for
/// use and clipPath) and the gradients, read once before painting.
pub(super) struct Defs<'a> {
    pub doc: &'a str,
    /// The viewBox [x, y, width, height], for user-space percentages.
    pub view: [f32; 4],
    pub ids: Vec<(&'a str, usize)>,
    pub grads: Vec<(&'a str, Gradient<'a>)>,
}

impl<'a> Defs<'a> {
    pub fn scan(doc: &'a str, view: [f32; 4]) -> Defs<'a> {
        let mut d = Defs { doc, view, ids: Vec::new(), grads: Vec::new() };
        let mut pos = 0;
        while let Some((tag, next)) = next_tag(doc, pos) {
            if !tag.closing {
                if let Some(id) = attr(tag.attrs, "id").filter(|_| d.ids.len() < MAX_IDS) {
                    d.ids.push((id, pos));
                    let radial = tag.name == "radialGradient";
                    if (radial || tag.name == "linearGradient") && d.grads.len() < MAX_GRADIENTS {
                        let g = read_gradient(doc, radial, tag.attrs, tag.self_closing, next);
                        d.grads.push((id, g));
                    }
                }
            }
            pos = next;
        }
        d
    }

    /// Where the element with `id` starts: a position whose next tag is it.
    pub fn element(&self, id: &str) -> Option<usize> {
        self.ids.iter().find(|(k, _)| *k == id).map(|(_, at)| *at)
    }

    /// The index of gradient `id`.
    pub fn gradient(&self, id: &str) -> Option<usize> {
        self.grads.iter().position(|(k, _)| *k == id)
    }
}

/// The id a `url(#id)` reference or an `#id` href names.
pub(super) fn ref_id(v: &str) -> Option<&str> {
    let t = v.trim();
    let inner = t.strip_prefix("url(").and_then(|r| r.split_once(')')).map_or(t, |(i, _)| i);
    inner.trim().trim_matches(|c| c == '"' || c == '\'').strip_prefix('#')
}
