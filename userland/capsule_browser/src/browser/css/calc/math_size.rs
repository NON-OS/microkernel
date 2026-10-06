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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MathOp {
    Min,
    Max,
    /* clamp(lo, val, hi) = max(lo, min(val, hi)): lo wins over hi. */
    Clamp,
}

/// Most arguments one min()/max() keeps once its plain lengths are folded,
/// and the three of a clamp().
pub const MATH_ARGS: usize = 3;

/// A min()/max()/clamp() with a percentage in an argument, kept for layout;
/// each argument is (px, per-mille of the base), like `Size::Calc`, held in
/// 16 bits so a computed style stays small: every Size field carries one.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MathSize {
    pub op: MathOp,
    pub n: u8,
    pub args: [(i16, i16); MATH_ARGS],
}

impl MathSize {
    /// Decide the comparison against a containing `base` in px.
    pub fn resolve(&self, base: i32) -> i32 {
        let at = |(px, pml): (i16, i16)| {
            (px as i32)
                .saturating_add((base as i64 * pml as i64 / 1000).clamp(-1 << 30, 1 << 30) as i32)
        };
        let vals = self.args[..self.n as usize].iter().map(|a| at(*a));
        match self.op {
            MathOp::Min => vals.min().unwrap_or(0),
            MathOp::Max => vals.max().unwrap_or(0),
            MathOp::Clamp => at(self.args[1]).min(at(self.args[2])).max(at(self.args[0])),
        }
    }

    /// The same comparison scaled by `k`. A negative factor turns min into
    /// max and swaps a clamp's bounds, since negation reverses the order.
    pub(in crate::browser::css) fn scaled(self, k: f32) -> Option<MathSize> {
        let mut out = self;
        for (i, (px, pml)) in self.args[..self.n as usize].iter().enumerate() {
            out.args[i] = (lin(*px as f32 * k)?, lin(*pml as f32 * k)?);
        }
        if k < 0.0 {
            out.op = match self.op {
                MathOp::Min => MathOp::Max,
                MathOp::Max => MathOp::Min,
                MathOp::Clamp => {
                    out.args.swap(0, 2);
                    MathOp::Clamp
                }
            };
        }
        Some(out)
    }
}
