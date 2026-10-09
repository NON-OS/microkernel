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

use super::cx::Cx;
use super::form_candidate::control;
use super::form_missing::suffering;
use super::tree_scan::find_under;

/* Constraint validation from markup (HTML 4.10.20): Some(true) valid,
 * Some(false) invalid, None for an element barred from validation, one
 * that is not a submittable control, or one whose validity this matcher
 * cannot decide. A form or fieldset is invalid when a control inside it is,
 * and undecided when one is. */
pub(super) fn validity(cx: &Cx, id: usize) -> Option<bool> {
    let tag = cx.element(id)?.tag.as_str();
    match tag {
        "form" | "fieldset" => {
            let mut undecided = false;
            let bad = find_under(cx, id, |d| match control(cx, d).then(|| suffering(cx, d)) {
                Some(Some(s)) => s,
                Some(None) => {
                    undecided = true;
                    false
                }
                None => false,
            });
            match bad {
                Some(_) => Some(false),
                None if undecided || cx.exhausted() => None,
                None => Some(true),
            }
        }
        _ if control(cx, id) => suffering(cx, id).map(|s| !s),
        _ => None,
    }
}
