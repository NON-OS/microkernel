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

use crate::browser::css::selector::{Pseudo, Selector};

use super::expand_merge::merged;
use super::seal::seal;

/* Copies an expansion may make of one selector. */
const MAX_COPIES: usize = 32;

/* Split :is() groups on the key compound into one selector per
 * alternative when every alternative is a single compound, so the rule
 * index buckets each copy by its own id, class or tag instead of testing
 * the rule on every element. The copies match exactly the elements the
 * original does, and each keeps the original specificity, which already
 * counts the :is() as its most specific argument. */
pub(super) fn expand(sel: Selector) -> Vec<Selector> {
    let flat = |l: &[Selector]| l.iter().all(|s| s.ancestors.is_empty() && s.element == 0);
    let groups = sel
        .key
        .pseudo
        .iter()
        .filter(|p| matches!(p, Pseudo::Matches(l) if !l.is_empty() && flat(l)));
    let copies = groups.fold(1usize, |n, p| match p {
        Pseudo::Matches(l) => n.saturating_mul(l.len()),
        _ => n,
    });
    if copies <= 1 || copies > MAX_COPIES {
        return vec![sel];
    }
    let mut base = sel.clone();
    let (split, keep): (Vec<Pseudo>, Vec<Pseudo>) = base
        .key
        .pseudo
        .drain(..)
        .partition(|p| matches!(p, Pseudo::Matches(l) if !l.is_empty() && flat(l)));
    base.key.pseudo = keep;
    let mut out = vec![base];
    for group in split {
        let Pseudo::Matches(alts) = group else { continue };
        out = out.iter().flat_map(|s| alts.iter().map(|a| merged(s, &a.key))).collect();
    }
    out.into_iter().map(seal).collect()
}
