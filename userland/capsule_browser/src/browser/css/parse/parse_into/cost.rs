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

use core::mem::size_of;

use crate::browser::css::selector::Selector;

/* Bytes one component of a selector keeps beyond its name: the String or
 * Vec header that holds it and the heap block behind that. */
const PART: usize = 64;

/* An estimate of what one selector parsed from `t` keeps, from its text
 * alone: the Selector, its names, and PART for each component (a class,
 * id, attribute, pseudo-class, combinator or compound). The parse budget
 * (cx) counts it. */
pub(super) fn selector_cost(t: &str) -> usize {
    let parts = t.bytes().filter(|b| b".#[:>+~ (".contains(b)).count() + 1;
    size_of::<Selector>() + t.len() + PART * parts
}
