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
//! One transfer for the client: the core set to its SPI mode and to the
//! fastest bit clock not above the one asked for, enabled, chip select
//! asserted for the whole exchange and deasserted after it whatever came,
//! and the core disabled again so no frame moves between requests.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::Deadline;

use crate::core::{exchange, scr_for, sscr0, sscr1, Regs32, E_IO};
use crate::facts::{INPUT_CLOCK_HZ, SSCR0, SSCR0_SSE, SSCR1, TRANSFER_MAX, TRANSFER_MS};
use crate::setup::{chip_select, Host};
use crate::wire::E_INVAL;

pub fn transfer(h: &Host, mode: u8, hz: u32, tx: &[u8]) -> Result<Vec<u8>, i32> {
    if mode > 3 || tx.is_empty() || tx.len() > TRANSFER_MAX {
        return Err(E_INVAL);
    }
    // A clock slower than the divider reaches is refused, never run faster.
    let scr = scr_for(INPUT_CLOCK_HZ, hz).ok_or(E_INVAL)?;
    let r = &h.regs;
    r.write32(SSCR0, sscr0(scr));
    r.write32(SSCR1, sscr1(mode));
    r.write32(SSCR0, sscr0(scr) | SSCR0_SSE);
    let mut rx = vec![0u8; tx.len()];
    let moved = chip_select(r, true).map_err(|_| E_IO).and_then(|()| {
        let deadline = Deadline::after_ms(TRANSFER_MS);
        exchange(r, tx, &mut rx, || !deadline.expired())
    });
    let _ = chip_select(r, false);
    r.write32(SSCR0, sscr0(scr));
    moved.map(|()| rx)
}
