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

use super::super::super::limits::{MAX_DEPTH, TRUNC_DEPTH};
use super::super::super::node::NodeKind;
use super::state::Builder;

/// An insertion location: under `parent`, before `before` or at the end.
#[derive(Clone, Copy)]
pub struct Loc {
    pub parent: usize,
    pub before: Option<usize>,
}

impl Loc {
    pub fn end(parent: usize) -> Self {
        Loc { parent, before: None }
    }
}

impl Builder {
    /// Put `id` into the tree at `loc`, the one way a node enters the tree
    /// while parsing. An element that would sit deeper than `MAX_DEPTH` goes
    /// in beside its would-be parent instead, as Blink does, so no page can
    /// nest the tree past what style and layout walk.
    pub(in super::super) fn link(&mut self, loc: Loc, id: usize) {
        let (mut parent, mut before) = (loc.parent, loc.before);
        let limit = match self.dom.nodes[id].kind {
            NodeKind::Element => MAX_DEPTH,
            _ => MAX_DEPTH + 1,
        };
        while parent != 0 && usize::from(self.depth[parent]) >= limit {
            parent = self.dom.nodes[parent].parent;
            before = None;
            self.dom.truncated |= TRUNC_DEPTH;
        }
        self.dom.nodes[id].parent = parent;
        let kids = &mut self.dom.nodes[parent].children;
        match before.and_then(|b| kids.iter().rposition(|&c| c == b)) {
            Some(i) => kids.insert(i, id),
            None => kids.push(id),
        }
        self.depth[id] = self.depth[parent].saturating_add(1);
    }

    /// Take `id` out of its parent, if it has one.
    pub(in super::super) fn unlink(&mut self, id: usize) {
        let parent = self.dom.nodes[id].parent;
        let kids = &mut self.dom.nodes[parent].children;
        if let Some(i) = kids.iter().rposition(|&c| c == id) {
            kids.remove(i);
        }
        self.dom.nodes[id].parent = 0;
    }
}
