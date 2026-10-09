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
use alloc::vec;
use alloc::vec::Vec;

use crate::browser::css::rule_index::key::hash_name;

use super::props::Props;
use super::resolver::{Own, PENDING};
use super::scope::{Var, VarScope};

/* The scope of element `id`, which declares the custom properties
 * `decls` (name, value) in cascade order over `parent`. The last
 * declaration of a name wins; values resolve against the element's own
 * set first, then the inherited scope. A property in a reference cycle,
 * or declared 'initial', has the invalid value. */
pub(in crate::browser::css) fn declare(
    parent: &Rc<VarScope>,
    decls: &[(&str, &str)],
    props: &Props,
    id: usize,
) -> Rc<VarScope> {
    let mut raw: Vec<(u64, &str, &str)> =
        decls.iter().map(|&(n, v)| (hash_name(n, true), n, v)).collect();
    raw.sort_by_key(|r| r.0);
    raw.reverse();
    raw.dedup_by(|b, a| a.0 == b.0 && a.1.eq_ignore_ascii_case(b.1));
    raw.reverse();
    let n = raw.len();
    let mut own = Own {
        raw: &raw,
        state: vec![PENDING; n],
        vals: vec![None; n],
        parent,
        props,
        id,
        depth: 0,
    };
    let vars: Vec<Var> = (0..n)
        .map(|i| Var { hash: raw[i].0, name: Rc::from(raw[i].1), value: own.resolve(i), owner: id })
        .collect();
    VarScope::child(parent, vars)
}
