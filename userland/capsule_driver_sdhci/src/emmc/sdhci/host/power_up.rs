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

//! Bus power on and off.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::text::Line;
use super::super::power::Vdd;
use super::super::regs::{CLOCK_CONTROL, POWER_CONTROL, POWER_ON};
use super::engine::Host;
use super::limits::{POWER_TRIES, SETTLE_MS};

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// SD Bus Power on at `vdd`: the voltage chosen while power is off,
    /// then power set (Power Control, as recalled, unverified). Some Intel
    /// hosts drop the first power-on just after leaving D3 (observed in
    /// Linux sdhci-pci-core.c, unverified), so the bit is read back a
    /// millisecond after each write and written again, POWER_TRIES in all.
    pub fn power_up(&mut self, vdd: Vdd) -> EmmcResult<()> {
        let on = vdd.select() | POWER_ON;
        self.io.w8(POWER_CONTROL, 0);
        self.io.w8(POWER_CONTROL, vdd.select());
        let held = (0..POWER_TRIES).any(|_| {
            self.io.w8(POWER_CONTROL, on);
            self.pause(1);
            self.io.r8(POWER_CONTROL) & POWER_ON != 0
        });
        if !held {
            let now = self.io.r8(POWER_CONTROL);
            self.say(Line::new().s(b"bus power did not stay on (power ").hex(now as u64).s(b")"));
            return Err(EmmcError::NoPower);
        }
        self.vdd = Some(vdd);
        self.say(Line::new().s(b"bus power on at ").dec(vdd.millivolts() as u64).s(b" mV"));
        self.pause(SETTLE_MS);
        Ok(())
    }

    /// Clock and bus power off.
    pub fn power_off(&mut self) {
        self.io.w16(CLOCK_CONTROL, 0);
        self.io.w8(POWER_CONTROL, 0);
        self.vdd = None;
        self.sd_hz = 0;
    }
}
