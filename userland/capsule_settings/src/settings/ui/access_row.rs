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

//! One of the rows the compositor holds, on Accessibility or the displays'
//! on Appearance: what it sets, and its value now.

use nonos_app_skeleton::PaintBuffer;

use crate::settings::access::text::value;
use crate::settings::schema::access_knob::Knob;
use crate::settings::state::State;

use super::control_geom::right_edge;
use super::metrics::BODY_PX;
use super::theme::VALUE_FG;
use super::{row_label, text};

pub fn paint(
    fb: &mut PaintBuffer,
    state: &State,
    k: Knob,
    card_x: u32,
    card_w: u32,
    y: i32,
    h: u32,
) {
    row_label::paint(fb, card_x, y, h, k.label(), None);
    let top = text::centred_top(0, h, BODY_PX) + y;
    let v = match k.on_screens() {
        true => crate::settings::screens::text::value(&state.screens, k),
        false => value(&state.access, k),
    };
    text::right(fb, right_edge(card_x, card_w), top, v, VALUE_FG, BODY_PX);
}
