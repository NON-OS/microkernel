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

use crate::browser::css::rule::Rule;

use super::layer_tree::{node, post_order};

/* Give each rule its @layer's cascade rank (CSS Cascade 5). Layers are
 * ordered by first appearance, an '@layer a, b;' statement included,
 * among their siblings; a layer's sublayers rank below the layer's own
 * rules, and unlayered rules (UNLAYERED) above every layer. The ranks
 * are recomputed over all rules, so a later sheet's sublayer of an
 * earlier layer still lands in its place. */
pub(super) fn rank(rules: &mut [Rule]) {
    let layered = |r: &Rule| r.layer_name.is_some() || r.flags & Rule::LAYER_ORDER != 0;
    if !rules.iter().any(layered) {
        return;
    }
    /* (name segment, children) per layer; 0 is the unlayered root. */
    let mut tree: Vec<(Rc<str>, Vec<usize>)> = vec![(Rc::from(""), Vec::new())];
    for r in rules.iter().filter(|r| layered(r)) {
        if r.flags & Rule::LAYER_ORDER != 0 {
            for d in &r.decls {
                node(&mut tree, &d.value);
            }
        } else if let Some(p) = &r.layer_name {
            node(&mut tree, p);
        }
    }
    let mut ranks = vec![0u16; tree.len()];
    post_order(&tree, &mut ranks);
    ranks[0] = Rule::UNLAYERED;
    let mut last: Option<(Rc<str>, u16)> = None;
    for r in rules.iter_mut() {
        let Some(p) = r.layer_name.clone() else { continue };
        r.layer = match &last {
            Some((q, k)) if Rc::ptr_eq(q, &p) => *k,
            _ => ranks.get(node(&mut tree, &p)).copied().unwrap_or(0),
        };
        last = Some((p, r.layer));
    }
}
