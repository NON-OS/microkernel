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

use crate::browser::css::calc_factor::factor;
use crate::browser::css::calc_term::term;

use super::cursor::P;
use super::math_size::MathSize;

/* A math value: numbers scale, lengths add. A length is a pixel part plus a
 * per-mille-of-base part, so "100% - 80px" survives to layout intact. Math is
 * a min()/max()/clamp() whose arguments carry percentages, so it can only be
 * decided at layout, once the base is known. */
#[derive(Clone, Copy)]
pub(in crate::browser::css) enum V {
    Num(f32),
    Len { px: f32, pml: f32 },
    Math(MathSize),
}

/// Evaluate one whole CSS value: a number, a length, or a math function
/// (calc, min, max, clamp). Arithmetic is only legal inside a function, so
/// the top level is a single operand. None on any form the grammar does not
/// cover; the declaration then drops, never guesses.
pub(in crate::browser::css) fn eval_value(value: &str, em: f32) -> Option<V> {
    let mut p = P { s: value.trim().as_bytes(), i: 0, em, d: 0 };
    let v = factor(&mut p)?;
    p.skip_ws();
    (p.i == p.s.len()).then_some(v)
}

/// A sum of terms. Lengths add part by part, numbers add; a number plus a
/// length, or anything plus an undecided min/max/clamp, has no meaning here.
pub(in crate::browser::css) fn expr(p: &mut P) -> Option<V> {
    let mut acc = term(p)?;
    loop {
        p.skip_ws();
        let sign = match p.s.get(p.i) {
            Some(b'+') => 1.0,
            Some(b'-') => -1.0,
            _ => return Some(acc),
        };
        p.i += 1;
        acc = match (acc, term(p)?) {
            (V::Len { px: a, pml: b }, V::Len { px: c, pml: d }) => {
                V::Len { px: a + sign * c, pml: b + sign * d }
            }
            (V::Num(a), V::Num(c)) => V::Num(a + sign * c),
            _ => return None,
        };
    }
}
