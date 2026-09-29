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

use alloc::string::String;

use crate::browser::css::selector::Pseudo;

use super::after_element::after_element;
use super::compound_part::Compound;
use super::cursor::Cur;
use super::ident::ident;
use super::pseudo_element::{legacy_element, pseudo_element};
use super::pseudo_fn::functional;
use super::pseudo_names::plain;

/* One pseudo-class or pseudo-element, the cursor on its first ':'. Only the
 * name is lower-cased; a functional argument is read from the original
 * text, so :not(.Upper) keeps its class case. A name this engine does not
 * know makes the selector invalid rather than never-matching, as it does in
 * Chromium, so the whole rule drops. */
pub(super) fn pseudo(c: &mut Cur, out: &mut Compound) -> Option<()> {
    c.i += 1;
    let elem = c.eat(b':');
    c.comments();
    let name = ident(c)?.to_ascii_lowercase();
    let func = c.eat(b'(');
    if out.element != 0 {
        return after_element(c, out, elem, &name, func);
    }
    if elem {
        return pseudo_element(c, out, &name, func);
    }
    if !func {
        if let Some(code) = legacy_element(&name) {
            out.element = code;
            out.spec.c += 1;
            return Some(());
        }
    }
    let p = if func {
        functional(c, &name, &mut out.spec)?
    } else {
        out.spec.b += 1;
        plain(&name)?
    };
    /* :root is the document element, which in an HTML document is html;
     * keying it on that tag lets the rule index bucket it there. */
    if matches!(p, Pseudo::Root) && out.simple.tag.is_none() {
        out.simple.tag = Some(String::from("html"));
    }
    out.simple.pseudo.push(p);
    Some(())
}
