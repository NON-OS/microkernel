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

use crate::browser::css::decl::Decl;
use crate::browser::css::rule::Rule;
use crate::browser::css::vars::{declare, Props, VarScope};

use super::order::Order;

/* The custom-property scope of a box: its parent's, shared, when none of
 * its declarations defines a custom property; otherwise the parent's plus
 * the ones it declares, in cascade order (important ones last). */
pub(in crate::browser::css) fn custom_scope(
    order: &Order,
    parent: &Rc<VarScope>,
    props: &Props,
    id: usize,
) -> Rc<VarScope> {
    let custom = |d: &Decl| d.flags & Decl::CUSTOM != 0;
    let in_rules = |(rules, hits): (&[Rule], &[super::order::Hit])| {
        hits.iter().any(|h| rules.get(h.rule as usize).is_some_and(|r| r.flags & Rule::CUSTOM != 0))
    };
    let any = order.inline.iter().any(custom) || in_rules(order.author) || in_rules(order.ua);
    if !any {
        return parent.clone();
    }
    let mut decls: Vec<(&str, &str)> = Vec::new();
    order.each(Rule::CUSTOM, &mut |d| {
        if custom(d) {
            decls.push((&d.name, &d.value));
        }
    });
    declare(parent, &decls, props, id)
}
