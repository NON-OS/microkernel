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

use crate::about::data::uptime::{read_millis, split_dhms};
use crate::about::format::u64_decimal;

use super::super::card::{self, titled};
use super::super::kv::{kv, ROW_H};
use super::super::metrics::CARD_PAD;

const UNAVAILABLE: &[u8] = b"unavailable";

pub const HEIGHT: u32 = card::OVERHEAD + ROW_H * 5;

// The clock is one reading split five ways, so every row comes from the same
// millisecond: sampling per row would let the seconds disagree with the minutes
// on the frame where it ticks.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, w: u32) {
    let top = titled(fb, x, y, w, HEIGHT, b"Uptime");
    let ms = read_millis();
    let (d, h, m, s) = split_dhms(ms.unwrap_or(0));
    let mut raw = [0u8; 20];
    let mut bd = [0u8; 20];
    let mut bh = [0u8; 20];
    let mut bm = [0u8; 20];
    let mut bs = [0u8; 20];
    // An unanswered clock reads unavailable on every row: a column of zeros
    // would read as a machine that has just booted.
    let rows: [(&[u8], &[u8]); 5] = match ms {
        Some(v) => [
            (b"Monotonic ms", u64_decimal(v, &mut raw)),
            (b"Days", u64_decimal(d, &mut bd)),
            (b"Hours", u64_decimal(h, &mut bh)),
            (b"Minutes", u64_decimal(m, &mut bm)),
            (b"Seconds", u64_decimal(s, &mut bs)),
        ],
        None => [
            (b"Monotonic ms", UNAVAILABLE),
            (b"Days", UNAVAILABLE),
            (b"Hours", UNAVAILABLE),
            (b"Minutes", UNAVAILABLE),
            (b"Seconds", UNAVAILABLE),
        ],
    };
    for (i, (label, value)) in rows.into_iter().enumerate() {
        let row_y = top + (i as u32 * ROW_H) as i32;
        kv(fb, x + CARD_PAD, row_y, card::inner(w), label, value, ms.is_some());
    }
}
