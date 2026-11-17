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

//! Field: a role draws the text at its size, and the field is as much taller
//! than `CONTROL_H` as the role's line is taller than Body's, at every scale.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::CONTROL_H;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::widgets::{text, Field};

use super::canvas::Canvas;
use super::field_pick::typing;
use super::field_role::{ROLES, RUNS};
use super::serial::{serial, SCALES};

#[test]
fn field_role_sets_the_text_size_and_height() {
    let _g = serial();
    for (q, role) in SCALES.into_iter().flat_map(|q| ROLES.map(|r| (q, r))) {
        set_scale(q);
        let (s, run) = (scale(), RUNS[1]);
        let line = |r: TextRole| line_height(s.font(r.px())) as u32;
        let f = Field::new(run).role(role);
        let want = s.px(CONTROL_H) + line(role) - line(TextRole::Body);
        assert_eq!(f.size().1, want, "{role:?} height at {q}/4");
        let at = typing(f).1;
        let lit = Canvas::new(at, 0).painted(|fb| f.draw(fb, at));
        let bare = Canvas::new(at, 0).painted(|fb| Field::new("").role(role).draw(fb, at));
        let xs: Vec<u32> = lit.diff(&bare).into_iter().map(|p| p.0).collect();
        let drawn = xs.iter().max().unwrap() - xs.iter().min().unwrap() + 1;
        let w = text::width(run, s.font(role.px()));
        assert!(drawn.abs_diff(w) <= s.px(3), "{role:?} drew {drawn} for {w} at {q}/4");
    }
}
