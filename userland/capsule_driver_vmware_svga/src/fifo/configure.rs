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

//! Laying the FIFO out, as Linux's vmw_fifo_create does: the registers
//! first, the ring after at least a page, CONFIG_DONE dropped while the
//! words change and raised once they are in place.

use core::sync::atomic::{fence, Ordering};

use super::ring::Fifo;
use crate::constants::fifo::{
    BASIC_REGS, FIFO_BUSY, FIFO_MAX, FIFO_MIN, FIFO_NEXT_CMD, FIFO_STOP, MIN_RING, MIN_START,
};
use crate::constants::regs::{CAP_EXTENDED_FIFO, CONFIG_DONE, MEM_REGS, MEM_SIZE};
use crate::regs::Regs;

pub fn configure(regs: &Regs, fifo: &mut Fifo, caps: u32) -> Result<(), &'static str> {
    let extended = caps & CAP_EXTENDED_FIFO != 0;
    let reg_words = if extended { regs.read(MEM_REGS) } else { BASIC_REGS };
    let min = reg_words.saturating_mul(4).max(MIN_START);
    let max = regs.read(MEM_SIZE).min(fifo.bytes()) & !3;
    if max < min.saturating_add(MIN_RING) {
        return Err("FIFO too small for a ring");
    }
    regs.write(CONFIG_DONE, 0);
    fifo.set(FIFO_MIN, min);
    fifo.set(FIFO_MAX, max);
    fence(Ordering::Release);
    fifo.set(FIFO_NEXT_CMD, min);
    fifo.set(FIFO_STOP, min);
    if extended {
        fifo.set(FIFO_BUSY, 0);
    }
    fence(Ordering::SeqCst);
    regs.write(CONFIG_DONE, 1);
    (fifo.min, fifo.max) = (min, max);
    Ok(())
}
