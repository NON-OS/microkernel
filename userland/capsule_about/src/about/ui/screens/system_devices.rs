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
use nonos_libc::mk_boot_census;

use crate::about::data::census::{row, ROWS, VALUE};

use super::super::card::{self, titled};
use super::super::kv::{kv, ROW_H};
use super::super::metrics::CARD_PAD;

pub const HEIGHT: u32 = card::OVERHEAD + ROW_H * ROWS as u32;

/// The census's lines: eleven classes and the summary, with room.
const TEXT: usize = 2048;

// What the kernel counted at boot against what came up: each class of
// device, and why one is missing or not read. Before the kernel has said it,
// one row says so rather than a table of guesses.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, w: u32) {
    let top = titled(fb, x, y, w, HEIGHT, b"Devices");
    let mut text = [0u8; TEXT];
    let n = mk_boot_census(&mut text);
    if n <= 0 {
        let said: &[u8] = if n == 0 { b"being taken" } else { b"unavailable" };
        kv(fb, x + CARD_PAD, top, card::inner(w), b"Census", said, false);
        return;
    }
    let mut buf = [0u8; VALUE];
    let mut i = 0u32;
    for line in text[..n as usize].split(|b| *b == b'\n') {
        let Some((label, len)) = row(line, &mut buf) else { continue };
        if i as usize == ROWS {
            break;
        }
        kv(fb, x + CARD_PAD, top + (i * ROW_H) as i32, card::inner(w), label, &buf[..len], false);
        i += 1;
    }
}
