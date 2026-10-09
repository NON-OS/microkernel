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
use alloc::vec;
use alloc::vec::Vec;

use super::node::{Node, NodeKind, Ns};
use super::quirks::Quirks;

pub struct Dom {
    pub nodes: Vec<Node>,
    /// The address this document was loaded from.
    ///
    /// It belongs to the document rather than to the viewer: it is what
    /// `location` reports, and what a relative `href` or `src` resolves
    /// against. A script told the page came from somewhere else builds links
    /// that go somewhere else.
    pub base: String,
    /// Where each node was last laid out, as `[x, y, w, h]`.
    ///
    /// A page measures itself to decide what fits, and every one of those
    /// reads answered zero, which is indistinguishable from an element with
    /// no size. The numbers exist in the display list already; they are
    /// copied here because that is what a script can reach.
    pub rects: Vec<[i32; 4]>,
    /// The viewport the document is laid out in, width and height, which a
    /// script's matchMedia is judged against as the page's own @media is.
    pub viewport: (u32, u32),
    /// How far the page is scrolled down, which a script reads as
    /// window.scrollY and takes off getBoundingClientRect's top.
    pub scroll_y: u32,
    /// How tall the page was at its last layout, which a script's own
    /// scroll is held within.
    pub content_h: u32,
    /// The cascade result of each node at the last layout, which a
    /// script's getComputedStyle reads (style_facts).
    pub facts: Vec<super::style_facts::Facts>,
    /// The reader's session history as history.length reads it: how
    /// many entries it holds and which is shown (State::note_history).
    pub history: (u32, u32),
    /// What the parser left out, as `limits::TRUNC_*` bits: nodes past the
    /// node cap, attributes past the budget, nesting past the depth cap.
    pub truncated: u8,
    /// The rendering mode the doctype selected.
    pub quirks: Quirks,
}

impl Dom {
    pub fn new() -> Self {
        let root = Node {
            kind: NodeKind::Document,
            tag: String::new(),
            text: String::new(),
            attrs: Vec::new(),
            parent: 0,
            children: Vec::new(),
            ns: Ns::Html,
        };
        Dom {
            nodes: vec![root],
            base: String::new(),
            rects: Vec::new(),
            viewport: (0, 0),
            scroll_y: 0,
            content_h: 0,
            facts: Vec::new(),
            history: (0, 0),
            truncated: 0,
            quirks: Quirks::No,
        }
    }
}

impl Dom {
    /// Whether the parser left part of the page out: nodes past the node
    /// cap, or attributes past a tag's or the document's limit, which loses
    /// links, sources and classes. Nesting flattened past the depth cap is
    /// not counted: every node is still there, only placed beside rather
    /// than inside.
    pub fn cut_short(&self) -> bool {
        self.truncated & (super::limits::TRUNC_NODES | super::limits::TRUNC_ATTRS) != 0
    }
}

impl Default for Dom {
    /// A document holding nothing but its root, which is what `new` builds.
    fn default() -> Self {
        Self::new()
    }
}
