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
use super::math_size::{MathOp, MathSize, MATH_ARGS};

/// min() or max() over evaluated arguments. Numbers compare as numbers and
/// plain lengths fold to one length now. Arguments with a percentage part
/// are kept for layout, next to the folded plain length; a nested min in a
/// min (or max in a max) flattens. Anything else mixed in drops the value.
pub(super) fn min_max(op: MathOp, args: &[V]) -> Option<V> {
    let pick = |a: f32, b: f32| if op == MathOp::Min { a.min(b) } else { a.max(b) };
    let mut num: Option<f32> = None;
    let mut plain: Option<f32> = None;
    let mut out = MathSize { op, n: 0, args: [(0, 0); MATH_ARGS] };
    for a in args {
        match *a {
            V::Num(n) => num = Some(num.map_or(n, |m| pick(m, n))),
            V::Len { px, pml } if pml == 0.0 => plain = Some(plain.map_or(px, |q| pick(q, px))),
            V::Len { px, pml } => push(&mut out, (lin(px)?, lin(pml)?))?,
            V::Math(m) if m.op == op => {
                for arg in &m.args[..m.n as usize] {
                    push(&mut out, *arg)?;
                }
            }
            V::Math(_) => return None,
        }
    }
    match (num, plain, out.n) {
        (Some(n), None, 0) => Some(V::Num(n)),
        (None, Some(px), 0) => Some(V::Len { px, pml: 0.0 }),
        (None, plain, _) => {
            if let Some(px) = plain {
                push(&mut out, (lin(px)?, 0))?;
            }
            Some(match out.args[..out.n as usize] {
                [(px, pml)] => V::Len { px: px as f32, pml: pml as f32 },
                _ => V::Math(out),
            })
        }
        _ => None,
    }
}

pub(super) fn push(m: &mut MathSize, arg: (i16, i16)) -> Option<()> {
    let slot = m.args.get_mut(m.n as usize)?;
    *slot = arg;
    m.n += 1;
    Some(())
}
