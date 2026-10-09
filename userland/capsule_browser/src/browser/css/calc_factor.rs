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

use super::calc::{call, expr, number, MAX_CALC_DEPTH, P, V};

/// One operand: a parenthesised subexpression, a math function, or a number
/// with an optional unit. Percent becomes per-mille of the eventual base.
pub(super) fn factor(p: &mut P) -> Option<V> {
    p.skip_ws();
    match p.s.get(p.i)? {
        b'(' => {
            p.i += 1;
            if p.d >= MAX_CALC_DEPTH {
                return None;
            }
            p.d += 1;
            let v = expr(p)?;
            p.d -= 1;
            p.skip_ws();
            (p.s.get(p.i) == Some(&b')')).then(|| p.i += 1)?;
            Some(v)
        }
        b if b.is_ascii_alphabetic() => call(p),
        _ => number(p),
    }
}
