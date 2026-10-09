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

use crate::browser::css::selector::{Pseudo, Simple};

use super::compound_part::Compound;

use super::attr_test::attr;
use super::compound_part::id;
use super::cursor::Cur;
use super::ident::ident;
use super::pseudo::pseudo;
use super::spec::Spec;
use super::type_sel::type_selector;

/* One compound selector, scanned in a single pass: an optional type or
 * universal selector first, then ids, classes, attributes, pseudo-classes
 * and the nesting selector in any order, then at most one pseudo-element
 * with what may follow it. A pseudo segment ends where its name (and any
 * balanced argument) ends, so classes and ids after it still belong to the
 * compound. None when nothing is here or something malformed is. */
pub(super) fn compound(c: &mut Cur) -> Option<Compound> {
    let mut out = Compound { simple: Simple::empty(), spec: Spec::default(), element: 0 };
    let mut any = type_selector(c, &mut out)?;
    loop {
        c.comments();
        let Some(b) = c.peek() else { break };
        if out.element != 0 && b != b':' {
            break;
        }
        match b {
            b'#' => {
                c.i += 1;
                id(&mut out.simple, ident(c)?);
                out.spec.a += 1;
            }
            b'.' => {
                c.i += 1;
                out.simple.classes.push(ident(c)?);
                out.spec.b += 1;
            }
            b'[' => {
                attr(c, &mut out.simple)?;
                out.spec.b += 1;
            }
            /* The nesting selector outside any nested rule is :scope, with
             * no specificity of its own. */
            b'&' => {
                c.i += 1;
                out.simple.pseudo.push(Pseudo::Scope);
            }
            b':' => pseudo(c, &mut out)?,
            _ => break,
        }
        any = true;
    }
    any.then_some(out)
}
