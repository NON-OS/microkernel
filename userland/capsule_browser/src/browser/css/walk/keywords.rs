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

/* The CSS-wide keywords. Where a property's cascade ends on one, its value
 * is the one the element starts from before any declaration: the initial
 * value for a property that does not inherit, the parent's for one that
 * does (Computed::inherit_from). So a keyword needs no value of its own:
 * the earlier declarations it overrides are passed over instead. */

use super::keyword_table::inherits;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Kw {
    /* unset, and initial or inherit where it means the same. */
    Reset,
    /* revert: back to the UA origin, whose declarations stay. */
    Revert,
}

/* The keyword `value` is, read for property `name`; None for a value, and
 * for the cases that start value cannot stand for: inherit on a property
 * that does not inherit, initial on one that does. */
pub(super) fn keyword(name: &str, value: &str) -> Option<Kw> {
    let v = value.trim();
    let inherited = inherits(name);
    match () {
        _ if v.eq_ignore_ascii_case("unset") => Some(Kw::Reset),
        _ if v.eq_ignore_ascii_case("revert") || v.eq_ignore_ascii_case("revert-layer") => {
            Some(Kw::Revert)
        }
        _ if v.eq_ignore_ascii_case("inherit") && inherited => Some(Kw::Reset),
        _ if v.eq_ignore_ascii_case("initial") && !inherited => Some(Kw::Reset),
        _ => None,
    }
}

/* Whether a keyword on `cut` passes over a declaration of `name`: the same
 * property, a longhand of the shorthand `cut`, or anything but a custom
 * property under `all`. The flex shorthand sets grow, shrink and basis. */
pub(super) fn covers(cut: &str, name: &str) -> bool {
    match cut {
        _ if cut == name => true,
        "all" => !name.starts_with("--") && name != "direction",
        "flex" => matches!(name, "flex-grow" | "flex-shrink" | "flex-basis"),
        _ => name.len() > cut.len() && name.starts_with(cut) && name[cut.len()..].starts_with('-'),
    }
}
