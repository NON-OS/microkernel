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

use super::selector::Selector;

/* Selectors 4 specificity, counted by the parser from the selector as
 * written: ids; then classes, attributes and pseudo-classes (:root among
 * them); then types and pseudo-elements. :is(), :not() and :has() count as
 * their most specific argument, :where() as nothing, and nth-child(An+B of
 * S) as one pseudo-class plus S's most specific argument. CSS 2.1 6.4.3
 * compares the counts as a tuple, so they are packed ten bits per level,
 * each clamped, into one u32 that compares the same way: one id always
 * outranks any number of classes. */
pub fn specificity(sel: &Selector) -> u32 {
    sel.spec
}
