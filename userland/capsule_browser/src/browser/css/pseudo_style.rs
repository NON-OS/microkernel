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

use alloc::boxed::Box;
use alloc::string::String;

use crate::browser::dom::Dom;

use super::apply::apply_decl;
use super::budget::MatchBudget;
use super::computed::Computed;
use super::content_text::content_text;
use super::matching::{pseudo_hits, Siblings};
use super::rule::Rule;
use super::rule_index::RuleIndex;

/* One generated-content box: its content string and cascaded style, boxed so
 * the (None, None) slot every node carries costs 64 bytes instead of 1.3 KB. */
pub struct PseudoText {
    pub text: String,
    pub style: Box<Computed>,
}

/* The element's ::before and ::after boxes, each None unless a matching rule
 * declares displayable content. Both inherit from the host's final style. */
pub(super) fn pseudo_pair(
    dom: &Dom,
    sib: &Siblings,
    id: usize,
    author: (&[Rule], &RuleIndex),
    host: &Computed,
    vars: &[(String, String)],
    budget: &mut MatchBudget,
) -> (Option<PseudoText>, Option<PseudoText>) {
    let mut one = |which| {
        let hits = pseudo_hits(dom, sib, id, author, which, budget);
        let mut style = Computed::inherit_from(host);
        let mut text: Option<String> = None;
        for rule in hits.iter().filter_map(|&(_, i)| author.0.get(i)) {
            for d in &rule.decls {
                if d.name == "content" {
                    text = content_text(&d.value);
                } else {
                    apply_decl(&mut style, &d.name, &d.value, host.font_size_px, vars);
                }
            }
        }
        text.map(|text| PseudoText { text, style: Box::new(style) })
    };
    (one(1), one(2))
}
