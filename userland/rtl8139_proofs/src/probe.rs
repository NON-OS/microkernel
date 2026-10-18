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

//! What the Bus Master tests read the part through: its resets, both
//! engines, and whether both rings name this driver's frames.

use std::{cell::Cell, rc::Rc};

use nonos_libc::{mk_pio_read, Part};

use crate::constants::regs::{CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD, REG_RBSTART, REG_TXADDR0};
use crate::part::{PIO_GRANT, RX_DA, TX_DA};

/// Reads the part through the port, as the driver does.
pub fn probe(resets: &Rc<Cell<u32>>) -> impl Fn() -> Part + 'static {
    let resets = Rc::clone(resets);
    move || {
        let read = |offset, width| {
            let mut v = 0u32;
            (mk_pio_read(PIO_GRANT, offset, width, &mut v) == 0).then_some(v)
        };
        let cmd = read(REG_CMD, 1).unwrap_or(0) as u8;
        let rbstart = read(REG_RBSTART, 4) == Some(RX_DA as u32);
        Part {
            resets: resets.get(),
            rx_on: cmd & CMD_RX_ENABLE != 0,
            tx_on: cmd & CMD_TX_ENABLE != 0,
            rings_own: rbstart && read(REG_TXADDR0, 4) == Some(TX_DA as u32),
        }
    }
}
