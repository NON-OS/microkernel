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

use crate::browser::css::selector::{Pseudo, Selector, Simple};

/* A copy of `s` whose key also requires everything `alt` does. Two
 * different tags or ids can never both hold, which makes the copy one that
 * never matches rather than a wider one. */
pub(super) fn merged(s: &Selector, alt: &Simple) -> Selector {
    let mut out = s.clone();
    let k = &mut out.key;
    for (mine, theirs) in [(&mut k.tag, &alt.tag), (&mut k.id, &alt.id)] {
        match (mine.as_ref(), theirs) {
            (Some(a), Some(b)) if a != b => k.pseudo.push(Pseudo::Never),
            (None, Some(b)) => *mine = Some(b.clone()),
            _ => {}
        }
    }
    k.classes.extend(alt.classes.iter().cloned());
    k.attrs.extend(alt.attrs.iter().cloned());
    k.pseudo.extend(alt.pseudo.iter().cloned());
    out
}
