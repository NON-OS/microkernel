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

//! One entry of a list row, painted by its list's own painter, with the
//! hairline under it.

use nonos_app_skeleton::PaintBuffer;

use crate::settings::state::State;

use super::walk::Item;
use super::{bt_rows, net_rows, row, saved_rows, station_row};

pub fn paint(fb: &mut PaintBuffer, state: &State, item: Item, x: u32, w: u32, y: i32, h: u32) {
    match item {
        Item::Network(i) => net_rows::paint(fb, state, i, x, w, y, h),
        Item::Saved(i) => saved_rows::paint(fb, state, i, x, w, y, h),
        Item::BtDevice(i) => bt_rows::paint(fb, state, i, x, w, y, h),
        Item::Station => station_row::paint(fb, state, x, w, y, h),
        _ => return,
    }
    row::hairline(fb, x, w, y, h);
}
