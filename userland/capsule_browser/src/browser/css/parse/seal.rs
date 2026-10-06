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

use crate::browser::css::selector::{bloom_bits, name_hash, Pseudo, Selector, Simple};

use super::hoist::keys;

/* Fill in what the matcher reads without re-deriving: class and id keys on
 * every compound (after copying up what its :is() groups require), the
 * ancestor filter bits, and whether sibling positions are asked. Nested
 * argument lists were sealed when they were parsed. */
pub(super) fn seal(mut sel: Selector) -> Selector {
    keys(&mut sel.key);
    let mut bits = [0u64; 2];
    let mut positional = asks_position(&sel.key);
    for step in sel.ancestors.iter_mut() {
        keys(&mut step.simple);
        positional |= step.comb.is_sibling() || asks_position(&step.simple);
        /* A compound joined by a child or descendant combinator is always an
         * ancestor of the key; one joined by a sibling combinator is not. */
        if !step.comb.is_sibling() {
            let s = &step.simple;
            let tag = s.tag.as_deref().map(|t| name_hash(b't', t.as_bytes()));
            for key in tag.into_iter().chain(s.class_keys.iter().copied()).chain(nonzero(s.id_key))
            {
                let b = bloom_bits(key);
                bits = [bits[0] | b[0], bits[1] | b[1]];
            }
        }
    }
    sel.anc_bits = bits;
    sel.positional = positional;
    sel
}

fn nonzero(k: u64) -> Option<u64> {
    (k != 0).then_some(k)
}

fn asks_position(s: &Simple) -> bool {
    s.pseudo.iter().any(|p| match p {
        Pseudo::Matches(l) | Pseudo::Not(l) => l.iter().any(|x| x.positional),
        Pseudo::Has(_)
        | Pseudo::FirstChild
        | Pseudo::LastChild
        | Pseudo::OnlyChild
        | Pseudo::FirstOfType
        | Pseudo::LastOfType
        | Pseudo::OnlyOfType
        | Pseudo::NthChild(..)
        | Pseudo::NthLastChild(..)
        | Pseudo::NthOfType(..)
        | Pseudo::NthLastOfType(..)
        | Pseudo::NthChildOf(..)
        | Pseudo::NthLastChildOf(..) => true,
        _ => false,
    })
}
