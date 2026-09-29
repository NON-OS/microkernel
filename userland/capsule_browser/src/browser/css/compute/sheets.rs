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

use crate::browser::css::rule::Rule;
use crate::browser::css::rule_index::RuleIndex;
use crate::browser::css::vars::Props;

/* The author sheets, ranked by layer and indexed: plain selectors, the
 * pseudo-element ones, the registered custom properties, and the union
 * of the rules' flags. */
pub(in crate::browser::css) struct Author {
    pub rules: Vec<Rule>,
    pub index: RuleIndex,
    pub pseudo: RuleIndex,
    pub props: Props,
    pub flags: u16,
}

impl Author {
    pub fn new(mut rules: Vec<Rule>) -> Author {
        super::layers::rank(&mut rules);
        let index = RuleIndex::build(&rules, false);
        let pseudo = RuleIndex::build(&rules, true);
        let props = Props::from_rules(&rules);
        let flags = rules.iter().fold(0, |f, r| f | r.flags);
        Author { rules, index, pseudo, props, flags }
    }
}

/* What one cascade reads: the UA sheet and its index, the author
 * sheets, and the <noscript> policy inputs (QuickJS on, scripts ran). */
pub(in crate::browser::css) struct Inputs<'a> {
    pub ua: (&'a [Rule], &'a RuleIndex),
    pub author: &'a Author,
    pub js: (bool, Option<bool>),
}
