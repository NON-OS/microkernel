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

use alloc::vec::Vec;

use super::clamp::clamp;
use super::cursor::{MAX_CALC_DEPTH, P};
use super::expr::{expr, V};
use super::fold::min_max;
use super::math_size::MathOp;

/* More arguments than any real min()/max() list; bounds the allocation. */
const MAX_FN_ARGS: usize = 32;

/// The math function call at the cursor: calc(), min(), max() or clamp().
/// Arguments are comma-separated expressions, so the functions nest inside
/// each other and inside plain parentheses. Any other name is not a length.
pub(in crate::browser::css) fn call(p: &mut P) -> Option<V> {
    let start = p.i;
    while p.s.get(p.i).is_some_and(|b| b.is_ascii_alphabetic() || *b == b'-') {
        p.i += 1;
    }
    let name = p.s[start..p.i].to_ascii_lowercase();
    if p.s.get(p.i) != Some(&b'(') || p.d >= MAX_CALC_DEPTH {
        return None;
    }
    p.i += 1;
    p.d += 1;
    let mut args: Vec<V> = Vec::new();
    loop {
        if args.len() == MAX_FN_ARGS {
            return None;
        }
        args.push(expr(p)?);
        p.skip_ws();
        match p.s.get(p.i) {
            Some(b',') => p.i += 1,
            Some(b')') => break,
            _ => return None,
        }
    }
    p.i += 1;
    p.d -= 1;
    match &name[..] {
        b"calc" | b"-webkit-calc" if args.len() == 1 => Some(args[0]),
        b"min" => min_max(MathOp::Min, &args),
        b"max" => min_max(MathOp::Max, &args),
        b"clamp" => clamp(&args),
        _ => None,
    }
}
