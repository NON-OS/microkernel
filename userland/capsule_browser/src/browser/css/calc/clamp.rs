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

use super::cursor::lin;
use super::expr::V;
use super::math_size::{MathOp, MathSize};

/// clamp(lo, val, hi). Three numbers or three plain lengths decide now; with
/// a percentage in any of them the clamp waits for layout.
pub(super) fn clamp(args: &[V]) -> Option<V> {
    let [lo, val, hi] = *args else { return None };
    let part = |v: V| match v {
        V::Len { px, pml } => Some((px, pml)),
        _ => None,
    };
    if let (V::Num(a), V::Num(b), V::Num(c)) = (lo, val, hi) {
        return Some(V::Num(b.min(c).max(a)));
    }
    let (lo, val, hi) = (part(lo)?, part(val)?, part(hi)?);
    if lo.1 == 0.0 && val.1 == 0.0 && hi.1 == 0.0 {
        return Some(V::Len { px: val.0.min(hi.0).max(lo.0), pml: 0.0 });
    }
    let arg = |(px, pml): (f32, f32)| Some((lin(px)?, lin(pml)?));
    let args = [arg(lo)?, arg(val)?, arg(hi)?];
    Some(V::Math(MathSize { op: MathOp::Clamp, n: 3, args }))
}
