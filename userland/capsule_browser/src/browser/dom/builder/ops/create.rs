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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::html::tokenizer::Tag;

use super::super::super::limits::{MAX_NODES, TRUNC_NODES};
use super::super::super::node::{Node, NodeKind, Ns};
use super::state::Builder;

impl Builder {
    /// Create an element for a token, not yet in the tree. None once the node
    /// cap is reached, which also ends the parse.
    pub(in super::super) fn create(&mut self, tag: Tag, ns: Ns) -> Option<usize> {
        let attrs = self.budget(tag.attrs);
        self.new_node(NodeKind::Element, tag.name.into_owned(), attrs, ns)
    }

    /// Create a text node holding `text`.
    pub(in super::super) fn create_text(&mut self, text: String) -> Option<usize> {
        let id = self.new_node(NodeKind::Text, String::new(), Vec::new(), Ns::Html)?;
        self.dom.nodes[id].text = text;
        Some(id)
    }

    fn new_node(
        &mut self,
        kind: NodeKind,
        tag: String,
        attrs: Vec<(String, String)>,
        ns: Ns,
    ) -> Option<usize> {
        if self.dom.nodes.len() >= MAX_NODES {
            self.dom.truncated |= TRUNC_NODES;
            self.stopped = true;
            return None;
        }
        let id = self.dom.nodes.len();
        let children = Vec::new();
        let node = Node { kind, tag, text: String::new(), attrs, parent: 0, children, ns };
        self.dom.nodes.push(node);
        self.flags.push(0);
        self.depth.push(0);
        Some(id)
    }

    /// Keep the attributes that fit the document's budget and drop the rest.
    fn budget(&mut self, mut attrs: Vec<(String, String)>) -> Vec<(String, String)> {
        let mut keep = 0;
        while keep < attrs.len() && self.charge(attrs[keep].0.len() + attrs[keep].1.len()) {
            keep += 1;
        }
        attrs.truncate(keep);
        attrs.shrink_to_fit();
        attrs
    }
}
