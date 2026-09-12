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

//! The figures that are facts rather than tests.
//!
//! Nothing here can be wrong, so nothing here gets a verdict. They are shown
//! because the reach they describe is worth knowing, not because they prove
//! anything, and mixing them in with the checks above would have lent them a
//! green tick they have not earned.

use nonos_app_skeleton::PaintBuffer;

use crate::about::data::verify::Census;
use crate::about::format::{hex_u64, u64_decimal};

use super::super::kv::{kv, ROW_H};
use super::super::metrics::CARD_PAD;

pub(super) const CENSUS_ROWS: u32 = 4;

const LABELS: [&[u8]; CENSUS_ROWS as usize] =
    [b"Capsules running", b"Reaching disk", b"Reaching hardware", b"This window's mask"];

pub(super) fn census(fb: &mut PaintBuffer, y: i32, inner: u32, c: &Census) {
    let mut a = [0u8; 20];
    let mut b = [0u8; 20];
    let mut d = [0u8; 20];
    let mut mask = [0u8; 20];
    let values: [&[u8]; CENSUS_ROWS as usize] = [
        u64_decimal(c.capsules as u64, &mut a),
        u64_decimal(c.filesystem as u64, &mut b),
        u64_decimal(c.raw_hardware as u64, &mut d),
        hex_u64(c.own_mask, &mut mask),
    ];
    for (i, value) in values.into_iter().enumerate() {
        let row_y = y + (i as u32 * ROW_H) as i32;
        kv(fb, CARD_PAD, row_y, inner, LABELS[i], value, true);
    }
}
