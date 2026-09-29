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

use crate::browser::css::selector::Pseudo;

use super::cx::Cx;

/* The positional pseudo-classes, from the element's place among its
 * element siblings. */
pub(super) fn structural(cx: &Cx, id: usize, p: &Pseudo) -> bool {
    let Some((pos, count, pos_ty, count_ty)) = cx.sib.position(cx.dom, id) else {
        return false;
    };
    match *p {
        Pseudo::FirstChild => pos == 1,
        Pseudo::LastChild => pos == count,
        Pseudo::OnlyChild => count == 1,
        Pseudo::FirstOfType => pos_ty == 1,
        Pseudo::LastOfType => pos_ty == count_ty,
        Pseudo::OnlyOfType => count_ty == 1,
        Pseudo::NthChild(a, b) => nth(a, b, pos),
        Pseudo::NthLastChild(a, b) => nth(a, b, count - pos + 1),
        Pseudo::NthOfType(a, b) => nth(a, b, pos_ty),
        Pseudo::NthLastOfType(a, b) => nth(a, b, count_ty - pos_ty + 1),
        _ => false,
    }
}

/* An+B holds for position i when i = A*k + B for some k >= 0. Worked in
 * i64: i - B spans more than i32 once B is a saturated literal, and
 * i32::MIN % -1 would trap. */
pub(super) fn nth(a: i32, b: i32, i: i32) -> bool {
    let (a, d) = (a as i64, i as i64 - b as i64);
    if a == 0 {
        d == 0
    } else {
        d % a == 0 && d / a >= 0
    }
}
