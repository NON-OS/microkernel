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

use super::after_rules::allowed;
use super::compound_part::Compound;
use super::cursor::Cur;
use super::element_code::*;
use super::skip::skip_arg;

/* What may follow a pseudo-element, as Blink accepts it: ::marker after
 * ::before or ::after, ::before or ::after after ::part() or ::slotted(),
 * :is() and :where() after any, user-action pseudo-classes after the custom
 * -webkit- elements, ::file-selector-button, ::part() and ::cue, scrollbar
 * pseudo-classes after scrollbar parts, :window-inactive after ::selection
 * and :only-child after view transitions. Anything else is invalid. What
 * follows describes the pseudo-element, not its host, so it adds no host
 * condition; a ::before or ::after it narrows is no longer styled. */
pub(super) fn after_element(
    c: &mut Cur,
    out: &mut Compound,
    elem: bool,
    name: &str,
    func: bool,
) -> Option<()> {
    let code = out.element;
    if !allowed(code, elem, name, func) {
        return None;
    }
    if func {
        skip_arg(c, false);
        while c.eat(b',') {
            skip_arg(c, false);
        }
        if !c.close() {
            return None;
        }
    }
    if elem {
        out.spec.c += 1;
        out.element = OTHER;
    } else {
        out.spec.b += 1;
        if matches!(code, BEFORE | AFTER) {
            out.element = OTHER;
        }
    }
    Some(())
}
