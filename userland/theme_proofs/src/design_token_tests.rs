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

//! The NØNOS design tokens (HANDOFF/ui/identity.md) hold their own rules: the
//! brand's 1.2 type ramp on the 4 px grid, the spacing scale, short ordered
//! motion, and two curves that never go back on themselves.

use crate::design::easing::{apply, Curve, UNIT};
use crate::design::motion::{leave_ms, MOVE_MS, QUICK_MS, SETTLE_MS};
use crate::design::spacing::SPACE;
use crate::design::typography::TextStyle as T;

fn ramp() -> [T; 6] {
    [T::label(), T::caption(), T::body(), T::heading(), T::lead(), T::title()]
}

#[test]
fn the_type_ramp_steps_by_the_brand_ratio_on_the_grid() {
    let r = ramp();
    for w in r.windows(2) {
        let step = w[1].px as f32 / w[0].px as f32;
        // One step of 1.2, or two for the title, each within rounding.
        assert!((1.14..=1.26).contains(&step) || (1.38..=1.5).contains(&step), "step {step:.2}");
    }
    for s in r {
        let (px, line) = (s.px as f32, s.line_height);
        assert!(line % 4 == 0 && line as f32 >= px * 1.14, "{px} on {line}");
        assert!(s.px >= 14 || s.letter_spacing > 0, "only the tracked label goes under 14");
        assert!(s.px >= 12, "nothing under 12");
        assert!(s.px != 17 || s.line_height == 24, "the brand's 17 body on 24");
    }
}

#[test]
fn spacing_is_the_scale_and_only_the_scale() {
    let on_scale = SPACE.windows(2).all(|w| w[0] < w[1]) && SPACE.iter().all(|s| s % 4 == 0);
    assert!(on_scale && SPACE[..2] == [4, 8], "a half step, a unit, then the scale");
}

#[test]
fn motion_is_short_and_ordered() {
    let all = [QUICK_MS, SETTLE_MS, MOVE_MS];
    assert!(all.windows(2).all(|w| w[0] < w[1]) && all[2] <= 300, "{all:?}");
    for d in all {
        assert!(leave_ms(d) < d && leave_ms(d) > 0, "leaving is quicker than arriving");
    }
}

#[test]
fn settle_and_leave_run_from_start_to_end_and_never_back() {
    for c in [Curve::Settle, Curve::Leave] {
        assert_eq!((apply(c, 0), apply(c, UNIT)), (0, UNIT));
        assert!((1..=UNIT).all(|t| apply(c, t) >= apply(c, t - 1)), "{c:?} goes back");
    }
    for t in 1..UNIT {
        assert!(apply(Curve::Settle, t) >= t && apply(Curve::Leave, t) <= t);
        let m = apply(Curve::Settle, t) as i32 + apply(Curve::Leave, UNIT - t) as i32 - UNIT as i32;
        assert!(m.abs() <= 1, "not mirrors at {t}");
    }
}
