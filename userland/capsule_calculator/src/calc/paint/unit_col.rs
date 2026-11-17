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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Row, State as Look};

use crate::calc::convert::list;
use crate::calc::event::hover::look;
use crate::calc::hit::Hit;
use crate::calc::state::State;
use crate::calc::ui::convert_geom::{col_top, head_h, row};
use crate::calc::ui::convert_hit::ConvertHit;
use crate::calc::ui::metrics::metrics;

fn probe(from: bool, i: usize) -> Hit {
    if from {
        Hit::Convert(ConvertHit::From(i))
    } else {
        Hit::Convert(ConvertHit::To(i))
    }
}

pub fn paint(state: &State, fb: &mut PaintBuffer, from: bool) {
    let win_w = fb.width as i32;
    let head = if from { "From" } else { "To" };
    let selected = if from { state.from } else { state.to };
    let body = metrics().px_body;
    let top = text::centre_top(col_top() as u32, head_h() as u32, body);
    text::left(fb, row(win_w, from, 0).x, top, head, roles().muted, body);
    for (i, unit) in list(state.cat).iter().enumerate() {
        let st = Look { selected: i == selected, ..look(state, probe(from, i)) };
        Row::new(unit.name).state(st).draw(fb, row(win_w, from, i));
    }
}
