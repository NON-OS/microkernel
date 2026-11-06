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
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::{Button, Pill, State as Look};

use super::result;
use super::unit_col;
use crate::calc::convert::CATEGORIES;
use crate::calc::event::hover::look;
use crate::calc::hit::Hit;
use crate::calc::state::State;
use crate::calc::ui::convert_geom::{chip, swap};
use crate::calc::ui::convert_hit::ConvertHit;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    chips(state, fb);
    unit_col::paint(state, fb, true);
    unit_col::paint(state, fb, false);
    let st = look(state, Hit::Convert(ConvertHit::Swap));
    Button::new("<->").state(st).draw(fb, swap(fb.width as i32));
    result::paint(state, fb);
}

/// A pill per category, centred in its slot of the strip, the chosen one
/// selected. A click anywhere in the slot picks it, as before.
fn chips(state: &State, fb: &mut PaintBuffer) {
    for (i, cat) in CATEGORIES.iter().enumerate() {
        let slot = chip(fb.width as i32, i);
        let st = look(state, Hit::Convert(ConvertHit::Chip(i)));
        let pill = Pill::new(cat.label()).state(Look { selected: state.cat == *cat, ..st });
        pill.draw(fb, centre(slot, pill.size().0, slot.h));
    }
}
