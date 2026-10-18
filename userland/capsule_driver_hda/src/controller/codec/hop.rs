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

//! Which inputs of a widget a route may pass through.

use super::widget::Widget;
use crate::constants::{AMP_INDEX_COUNT, WIDGET_TYPE_PIN, WIDGET_TYPE_SELECTOR};

/// A widget that passes one of its inputs, chosen by SET_CONNECT_SEL.
pub fn exclusive(w: &Widget) -> bool {
    let ty = w.ty();
    (ty == WIDGET_TYPE_SELECTOR || ty == WIDGET_TYPE_PIN) && w.n_conn > 1
}

/// A widget whose input amp the route through it opens (`program`): a mixer
/// or selector with input amps. A pin's input amp is on its input side.
pub fn opens_input_amp(w: &Widget) -> bool {
    w.has_in_amp() && w.ty() != WIDGET_TYPE_PIN
}

/// Whether input `i` of `w` can be on a route: its input amp must be one the
/// amp verb can name, and on a widget that passes one input it must be the
/// input `fixed` that another output already has it pass.
pub fn usable(w: &Widget, i: usize, fixed: Option<u8>) -> bool {
    let named = !opens_input_amp(w) || i < AMP_INDEX_COUNT as usize;
    named && (!exclusive(w) || fixed.is_none_or(|f| f as usize == i))
}
