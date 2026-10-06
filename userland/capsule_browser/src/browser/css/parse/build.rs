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

use crate::browser::css::selector::{Comb, Pseudo, Selector, Simple, Step};

use super::compound_part::Compound;
use super::seal::seal;

/* A parsed run of compounds as a Selector read right to left: the last
 * compound is the key, each earlier one a step carrying the combinator on
 * its right. A relative selector gets one more step, joined by its leading
 * combinator (a descendant one when none was written), whose compound only
 * the :has() subject satisfies. Specificity sums the compounds. */
pub(super) fn build(
    mut parts: Vec<Compound>,
    combs: Vec<Comb>,
    lead: Option<Option<Comb>>,
) -> Option<Selector> {
    let key = parts.pop()?;
    let mut spec = key.spec;
    let mut ancestors: Vec<Step> = Vec::with_capacity(parts.len() + 1);
    for (part, comb) in parts.into_iter().zip(combs).rev() {
        spec = spec.add(part.spec);
        ancestors.push(Step { simple: part.simple, comb });
    }
    if let Some(lead) = lead {
        let mut anchor = Simple::empty();
        anchor.pseudo.push(Pseudo::HasAnchor);
        ancestors.push(Step { simple: anchor, comb: lead.unwrap_or(Comb::Descendant) });
    }
    let (element, spec) = (key.element, spec.pack());
    Some(seal(Selector {
        key: key.simple,
        ancestors,
        element,
        spec,
        anc_bits: [0; 2],
        positional: false,
    }))
}
