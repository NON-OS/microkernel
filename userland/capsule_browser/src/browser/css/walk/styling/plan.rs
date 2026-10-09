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

use alloc::vec;
use alloc::vec::Vec;

use crate::browser::css::decl::Decl;

use super::super::keyword_table::initial;
use super::super::keywords::{covers, keyword, Kw};
use super::super::order::Order;

/* What becomes of one declaration, in cascade order. */
#[derive(Clone, Copy)]
pub(super) enum Step {
    Apply,
    Skip,
    /* A keyword on a longhand: its initial value, over any shorthand. */
    Put(&'static str),
}

/* Each declaration's step; empty, all applying, when no keyword occurs.
 * Read from the highest precedence down, a keyword passes over what comes
 * before it that it covers (revert: all but the UA's). */
pub(super) fn plan<'a>(all: &[&'a Decl], order: &Order) -> Vec<Step> {
    if !all.iter().any(|d| keyword(&d.name, &d.value).is_some()) {
        return Vec::new();
    }
    let mut steps = vec![Step::Apply; all.len()];
    let mut cuts: Vec<(&'a str, Kw)> = Vec::new();
    for (i, d) in all.iter().enumerate().rev() {
        if let Some((_, k)) = cuts.iter().find(|(c, _)| covers(c, &d.name)) {
            if *k == Kw::Reset || !from_ua(order, d) {
                steps[i] = Step::Skip;
                continue;
            }
        }
        if let Some(k) = keyword(&d.name, &d.value) {
            steps[i] = initial(&d.name).map_or(Step::Skip, Step::Put);
            cuts.push((d.name.as_str(), k));
        }
    }
    steps
}

fn from_ua(order: &Order, d: &Decl) -> bool {
    let (rules, hits) = order.ua;
    let mut rs = hits.iter().filter_map(|h| rules.get(h.rule as usize));
    rs.any(|r| r.decls.as_ptr_range().contains(&(d as *const Decl)))
}
