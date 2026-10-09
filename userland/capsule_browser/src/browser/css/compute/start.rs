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

use crate::browser::css::budget::MatchBudget;
use crate::browser::css::matching::Siblings;
use crate::browser::css::rule::Rule;
use crate::browser::css::walk::{Counters, Out, Sheet, Walker};
use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::sheets::Inputs;
use super::styles::Styles;

impl Inputs<'_> {
    /* A cascade walk over `dom` with these sheets, its output sized for
     * every node. */
    pub fn walker<'w>(&'w self, dom: &'w Dom, sib: &'w Siblings, noscript: bool) -> Walker<'w> {
        let n = dom.nodes.len();
        let a = self.author;
        let counted = a.flags & Rule::COUNTERS != 0 || !a.pseudo.is_empty();
        let body = dom.nodes.iter().find(|n| n.kind == NodeKind::Element && n.tag == "body");
        let mut grids = Vec::new();
        grids.resize_with(n, || None);
        let out = Out {
            styles: Styles::new(n),
            bg_images: vec![None; n],
            svg_paint: vec![None; n],
            grids,
            pseudos: Vec::new(),
        };
        Walker {
            dom,
            sib,
            ua: Sheet { rules: self.ua.0, index: self.ua.1 },
            author: Sheet { rules: &a.rules, index: &a.index },
            pseudo: &a.pseudo,
            props: &a.props,
            budget: MatchBudget::new(),
            noscript,
            counters: counted.then(Counters::default),
            link: body.and_then(|b| b.attr("link")).map(String::from),
            buckets: Vec::new(),
            ua_hits: Vec::new(),
            author_hits: Vec::new(),
            out,
        }
    }
}
