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

use super::super::computed::Computed;
use super::super::rule::Rule;
use super::super::vars::VarScope;
use super::super::walk::{Hit, Walker};
use super::exists::exists;
use super::one::one;
use super::PseudoText;

/* The pseudo-elements of element `id` except ::after, which the walk
 * finishes with `after` once the children are done; its hits come back. */
#[inline(never)]
pub(in crate::browser::css) fn before(
    w: &mut Walker,
    id: usize,
    host: &Computed,
    scope: &Rc<VarScope>,
) -> Option<Vec<Hit>> {
    if w.pseudo.is_empty() {
        return None;
    }
    w.match_author(id, true);
    let hits = core::mem::take(&mut w.author_hits);
    let mut after = None;
    for group in hits.chunk_by(|a, b| a.elem == b.elem) {
        /* A ::before or ::after no rule gives content has none: no box.
         * The universal box-sizing rules most sheets carry match every
         * element's, and cost nothing here. */
        let rules = w.author.rules;
        let content = group
            .iter()
            .any(|h| rules.get(h.rule as usize).is_some_and(|r| r.flags & Rule::CONTENT != 0));
        let kind = group[0].elem;
        if !exists(kind, &w.dom.nodes[id], host, content) {
            continue;
        }
        match kind {
            PseudoText::AFTER => after = Some(group.to_vec()),
            _ => one(w, id, host, scope, group),
        }
    }
    w.author_hits = hits;
    after
}

#[inline(never)]
pub(in crate::browser::css) fn after(
    w: &mut Walker,
    id: usize,
    host: &Computed,
    scope: &Rc<VarScope>,
    hits: Vec<Hit>,
) {
    one(w, id, host, scope, &hits);
}
