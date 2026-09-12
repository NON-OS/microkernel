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

//! A BAR0 window and the part that answers from it. Rings are in `memory`.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::constants::regs::{CMD_RESET, REG_CMD, REG_MAC0};
use crate::constants::MAC_LEN;

/// What the part shows in the IDR bytes before anything programs them.
pub const FACTORY: [u8; MAC_LEN] = [0x00, 0xE0, 0x4C, 0x11, 0x22, 0x33];

pub fn window() -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(0x100));
    for (i, b) in FACTORY.iter().enumerate() {
        bar.present8(REG_MAC0 + i, *b);
    }
    bar
}

pub fn idr(bar: &FakeBar) -> [u8; MAC_LEN] {
    let mut out = [0u8; MAC_LEN];
    for (i, b) in out.iter_mut().enumerate() {
        *b = bar.wrote8(REG_MAC0 + i);
    }
    out
}

/// A part that completes a software reset on the next look.
pub fn resetting_part(bar: &FakeBar) {
    let cmd = bar.wrote8(REG_CMD);
    if cmd & CMD_RESET != 0 {
        bar.present8(REG_CMD, cmd & !CMD_RESET);
    }
}
