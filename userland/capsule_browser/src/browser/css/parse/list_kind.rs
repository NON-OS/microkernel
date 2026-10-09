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

use crate::browser::css::selector::Selector;

/* How a parenthesized selector list treats its arguments. Forgiving (:is,
 * :where) drops an argument that fails to parse; Strict (:not), Relative
 * (:has, whose arguments may open with a combinator), Compound (one
 * compound each, as :-webkit-any takes) and Of (nth-child's `of S`) make
 * the whole selector invalid instead. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind {
    Forgiving,
    Strict,
    Relative,
    Compound,
    Of,
}

/* Whether a parsed argument is one this kind of list accepts. */
pub(super) fn fits(kind: Kind, s: &Selector) -> bool {
    match kind {
        Kind::Forgiving | Kind::Of => true,
        Kind::Compound => s.element == 0 && s.ancestors.is_empty(),
        Kind::Strict | Kind::Relative => s.element == 0,
    }
}
