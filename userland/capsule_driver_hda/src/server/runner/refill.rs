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

use super::refill_log::{mark, REFILL_MARK, UNDERRUN_MARK};
use crate::audio::{PcmQueue, Period, Periods};
use crate::constants::SD_LPIB;
use crate::controller::bdl::{N_PERIODS, PERIOD_BYTES, RING_BYTES};
use crate::controller::dma_sync::flush;
use crate::controller::position::Position;
use crate::setup::Driver;

pub(super) struct Refill {
    wpos: usize,
    pub(super) periods: Periods,
    buffer: bool,
    position: Position,
}

impl Refill {
    /// `buffer`: the controller writes a DMA position buffer for the stream.
    pub(super) fn new(buffer: bool) -> Self {
        Refill { wpos: 0, periods: Periods::new(), buffer, position: Position::new(buffer) }
    }

    pub(super) fn reset(&mut self) {
        self.wpos = 0;
        self.periods = Periods::new();
        self.position = Position::new(self.buffer);
    }
}

pub(super) fn refill(driver: &Driver, q: &mut PcmQueue, rf: &mut Refill) {
    let lpib = unsafe { driver.regs.r32(driver.stream_off + SD_LPIB) };
    let buffer = match driver.posbuf_va {
        Some(va) => {
            flush(va, 8);
            unsafe { core::ptr::read_volatile(va as *const u32) }
        }
        None => 0,
    };
    let pos = rf.position.pick(buffer, lpib, RING_BYTES as u32) as u64;
    let rp = ((pos / PERIOD_BYTES) as usize) % N_PERIODS;
    while rf.wpos != rp {
        let base = driver.sample.user_va + (rf.wpos as u64) * PERIOD_BYTES;
        let slot =
            unsafe { core::slice::from_raw_parts_mut(base as *mut u8, PERIOD_BYTES as usize) };
        let filled = q.pop_into(slot);
        flush(base, PERIOD_BYTES);
        match rf.periods.period(filled, PERIOD_BYTES as usize) {
            Period::Played if rf.periods.played == 1 => mark(REFILL_MARK),
            Period::Underrun if rf.periods.underruns == 1 => mark(UNDERRUN_MARK),
            _ => {}
        }
        rf.wpos = (rf.wpos + 1) % N_PERIODS;
    }
}
