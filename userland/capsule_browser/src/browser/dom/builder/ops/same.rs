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

use alloc::borrow::Cow;

use crate::browser::html::tokenizer::Tag;

use super::state::Builder;

impl Builder {
    /// Whether two elements have the same tag name, namespace and attribute
    /// set, the Noah's Ark comparison. Attributes are compared as a set; the
    /// common case, the same attributes in the same order, is one pass.
    pub(in super::super) fn same_element(&self, a: usize, b: usize) -> bool {
        let (x, y) = (&self.dom.nodes[a], &self.dom.nodes[b]);
        if x.tag != y.tag || x.ns != y.ns || x.attrs.len() != y.attrs.len() {
            return false;
        }
        x.attrs == y.attrs || x.attrs.iter().all(|pair| y.attrs.contains(pair))
    }

    /// The token an element was created for, to create another like it.
    pub(in super::super) fn token_of(&self, id: usize) -> Tag<'static> {
        let n = &self.dom.nodes[id];
        Tag { name: Cow::Owned(n.tag.clone()), attrs: n.attrs.clone(), self_closing: false }
    }
}
