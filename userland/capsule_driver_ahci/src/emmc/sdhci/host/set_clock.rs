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

//! The SD clock rate and the High Speed timing it runs under.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::text::Line;
use super::super::clock::divider;
use super::super::regs::{
    CLK_CARD_EN, CLK_INT_EN, CLK_INT_STABLE, CLOCK_CONTROL, HC_HIGH_SPEED, HOST_CONTROL,
};
use super::engine::Host;
use super::limits::CLOCK_STABLE_MS;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// The SD clock at the fastest divided rate not above `hz` (0 stops
    /// it), as sdhci_set_clock: clock off, divider and internal clock on,
    /// wait for stable, then the SD clock on. Returns the rate set.
    pub fn set_clock(&mut self, hz: u32) -> EmmcResult<u32> {
        self.io.w16(CLOCK_CONTROL, 0);
        self.sd_hz = 0;
        if hz == 0 {
            return Ok(0);
        }
        let div = divider(self.caps.base_hz(), hz, self.caps.spec());
        let mut clk = div.field | CLK_INT_EN;
        self.io.w16(CLOCK_CONTROL, clk);
        let d = Deadline::after(&self.clock, CLOCK_STABLE_MS);
        while self.io.r16(CLOCK_CONTROL) & CLK_INT_STABLE == 0 {
            if d.passed(&self.clock) {
                self.io.w16(CLOCK_CONTROL, 0);
                self.say(Line::new().s(b"internal clock never stable"));
                return Err(EmmcError::ClockUnstable);
            }
            self.clock.relax();
        }
        clk |= CLK_CARD_EN;
        self.io.w16(CLOCK_CONTROL, clk);
        self.sd_hz = div.hz;
        Ok(div.hz)
    }

    /// High Speed Enable, with the SD clock stopped while it changes and
    /// restarted at `hz`.
    pub fn set_timing(&mut self, high_speed: bool, hz: u32) -> EmmcResult<u32> {
        self.io.w16(CLOCK_CONTROL, self.io.r16(CLOCK_CONTROL) & !CLK_CARD_EN);
        let mut hc = self.io.r8(HOST_CONTROL) & !HC_HIGH_SPEED;
        if high_speed {
            hc |= HC_HIGH_SPEED;
        }
        self.io.w8(HOST_CONTROL, hc);
        self.hs = high_speed;
        self.set_clock(hz)
    }
}
