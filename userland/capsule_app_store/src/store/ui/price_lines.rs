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
//! A listing's price, and for a paid one the payee it pays, as the market's
//! commerce block says them.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::metrics::SMALL_PX;
use super::text;
use super::wrap_whole::wrap_whole;
use crate::store::market::Kec;
use crate::store::payee::payee_line;
use crate::store::price::{gas_line, price_line, Price};
use crate::store::theme::ACCENT;

/// Paints and returns the y to carry on from.
pub fn paint(fb: &mut PaintBuffer, p: &Price, left: u32, room: u32, mut top: i32) -> i32 {
    text::line(fb, left, top, &price_line(p), ACCENT, SMALL_PX);
    top += 20;
    if let Some(gas) = gas_line(p) {
        text::line(fb, left, top, gas, ACCENT, SMALL_PX);
        top += 20;
    }
    for part in payee_line(&Kec, p).map_or_else(Vec::new, |l| wrap_whole(&l, room, SMALL_PX)) {
        text::line(fb, left, top, &part, ACCENT, SMALL_PX);
        top += 20;
    }
    top
}
