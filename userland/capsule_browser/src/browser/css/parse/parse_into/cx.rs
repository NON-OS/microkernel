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

use alloc::rc::Rc;
use alloc::vec::Vec;
use core::mem::size_of;

use crate::browser::css::decl::Decl;
use crate::browser::css::rule::Rule;
use crate::browser::css::selector::Selector;

/* One parse: the rules found so far and where the parser stands. */
pub(in crate::browser::css::parse) struct Cx<'a> {
    pub rules: &'a mut Vec<Rule>,
    /* Address of the sheet text, so a rule records its byte offset. */
    pub origin: usize,
    /* The @layer path the parser is inside, None outside every layer. */
    pub layer: Option<Rc<str>>,
    /* Selectors and bytes the sheet may still add; 0 selectors is full. */
    pub left: usize,
    pub bytes: usize,
}

impl Cx<'_> {
    /* Byte offset of `s`, a slice of the sheet text: it names an
     * anonymous layer. */
    pub fn at(&self, s: &str) -> u32 {
        (s.as_ptr() as usize).saturating_sub(self.origin) as u32
    }

    pub fn full(&self) -> bool {
        self.left == 0
    }

    /* Add a data rule, which has no selectors. */
    pub fn statement(&mut self, decls: Vec<Decl>, flags: u16) {
        self.push(Vec::new(), decls, flags, 0);
    }

    /* Add a rule in the current layer; `cost` estimates what its
     * selectors keep (select::parse_list). */
    pub fn push(&mut self, selectors: Vec<Selector>, decls: Vec<Decl>, flags: u16, cost: usize) {
        let n = selectors.len().max(1);
        let cost = cost + size_of::<Rule>() + decls.iter().map(Decl::cost).sum::<usize>();
        if self.full() || n > self.left || cost > self.bytes {
            self.left = 0;
            return;
        }
        (self.left, self.bytes) = (self.left - n, self.bytes - cost);
        let mut rule = Rule::new(selectors, decls, flags);
        rule.layer_name = self.layer.clone();
        rule.cost = cost as u32;
        self.rules.push(rule);
    }
}
