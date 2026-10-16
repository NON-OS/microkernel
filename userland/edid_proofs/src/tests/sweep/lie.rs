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

//! One lie per rule a sound timing keeps, told to a plan that kept all.

use super::plan::Plan;
use super::widths::Widths;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lie {
    NoWidth,
    NoHeight,
    NoHSync,
    NoVSync,
    HPastBlank,
    VPastBlank,
    TooSlow,
    TooFast,
}

pub const ALL: [Lie; 8] = {
    use Lie::*;
    [NoWidth, NoHeight, NoHSync, NoVSync, HPastBlank, VPastBlank, TooSlow, TooFast]
};

/// Break `p` by `lie`. False when the encoding cannot say it.
pub fn tell(p: &mut Plan, lie: Lie, w: &Widths) -> bool {
    let step = w.clock_step_khz as u64;
    match lie {
        Lie::NoWidth => zero(p, 0, w),
        Lie::NoHeight => zero(p, 4, w),
        Lie::NoHSync => zero(p, 3, w),
        Lie::NoVSync => zero(p, 7, w),
        // One short of porch plus pulse; type VII cannot say a zero blank.
        Lie::HPastBlank => past(p, 1, w),
        Lie::VPastBlank => past(p, 5, w),
        Lie::TooSlow => {
            let k = (p.total() - 1) / 1_000 / step * step;
            p.khz = k as u32;
            k > 0
        }
        Lie::TooFast => {
            // The rate is floored, so one kilohertz over the total can
            // still read as exactly 1 kHz; go a millionth past it.
            let over = (p.total() * 1_000_001).div_ceil(1_000_000);
            let k = over.div_ceil(step) * step;
            p.khz = k as u32;
            k <= w.clock_max_khz as u64
        }
    }
}

fn past(p: &mut Plan, blank: usize, w: &Widths) -> bool {
    let need = p.t[blank + 1] + p.t[blank + 2];
    p.t[blank] = need - 1;
    need - 1 > 0 || w.can_zero
}

fn zero(p: &mut Plan, i: usize, w: &Widths) -> bool {
    p.t[i] = 0;
    w.can_zero
}
