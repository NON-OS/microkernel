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

use crate::browser::css::selector::{Comb, Selector};

use super::build::build;
use super::compound::compound;
use super::compound_part::Compound;
use super::cursor::Cur;

/* Compounds allowed in one complex selector, and in one top-level selector
 * counting every nested argument. Matching recurses once per compound, so
 * these bound its stack as the nesting limit bounds the rest. */
const MAX_COMPOUNDS: usize = 64;
const MAX_TOTAL: u32 = 1024;

/* One complex selector: compounds joined by '>', '+', '~' or whitespace,
 * ending at the end of the text, a ',' or a ')'. Combinators are only
 * recognised here, between compounds, so brackets, parentheses, strings and
 * escapes never split one. A relative selector (a :has() argument) may open
 * with a combinator and is anchored to the :has() subject through one last
 * step. Nothing may follow a compound that ends in a pseudo-element. */
pub(super) fn complex(c: &mut Cur, relative: bool) -> Option<Selector> {
    let mut lead = None;
    if relative {
        lead = Some(c.combinator());
        c.trivia();
    }
    let (mut parts, mut combs): (Vec<Compound>, Vec<Comb>) = (Vec::new(), Vec::new());
    loop {
        c.compounds += 1;
        if parts.len() >= MAX_COMPOUNDS || c.compounds > MAX_TOTAL {
            return None;
        }
        let part = compound(c)?;
        let ends_element = part.element != 0;
        parts.push(part);
        let ws = c.trivia();
        if matches!(c.peek(), None | Some(b',') | Some(b')')) {
            break;
        }
        match c.combinator() {
            Some(k) if !ends_element => {
                c.trivia();
                combs.push(k);
            }
            None if ws && !ends_element => combs.push(Comb::Descendant),
            _ => return None,
        }
    }
    build(parts, combs, lead)
}
