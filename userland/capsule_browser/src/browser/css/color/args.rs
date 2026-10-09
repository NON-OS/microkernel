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

use super::comp::comp;

/// One colour-function component: a number (an angle already in degrees),
/// a percentage, or the `none` keyword.
#[derive(Clone, Copy, PartialEq)]
pub(in crate::browser::css) enum Comp {
    Num(f64),
    Pct(f64),
    Missing,
}

impl Comp {
    /// The component with 100% standing for `full`; `none` is zero.
    pub(in crate::browser::css) fn scaled(self, full: f64) -> f64 {
        match self {
            Comp::Num(v) => v,
            Comp::Pct(p) => p * full / 100.0,
            Comp::Missing => 0.0,
        }
    }
}

/// Three channel components and the alpha, resolved to 0..1 (1 if absent).
pub(in crate::browser::css) struct Args {
    pub c: [Comp; 3],
    pub alpha: f64,
}

/// Split `a, b, c[, d]` or `a b c [/ d]`: the legacy comma form takes a
/// fourth component as alpha, the modern form only after the slash.
pub(in crate::browser::css) fn args(inner: &str) -> Option<Args> {
    let (main, slash) = match inner.split_once('/') {
        Some((m, a)) => (m, Some(a)),
        None => (inner, None),
    };
    let mut c = [Comp::Missing; 4];
    let mut n = 0;
    for tok in main.split(|ch: char| ch == ',' || ch.is_ascii_whitespace()) {
        if tok.is_empty() {
            continue;
        }
        if n == 4 {
            return None;
        }
        c[n] = comp(tok)?;
        n += 1;
    }
    let alpha = match (n, slash) {
        (3, Some(a)) => Some(comp(a.trim())?),
        (3, None) => None,
        (4, None) => Some(c[3]),
        _ => return None,
    };
    let alpha = alpha.map_or(1.0, |a| a.scaled(1.0)).clamp(0.0, 1.0);
    Some(Args { c: [c[0], c[1], c[2]], alpha })
}
