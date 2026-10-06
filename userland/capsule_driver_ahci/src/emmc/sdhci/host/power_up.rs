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
use super::limits::SETTLE_MS;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Bus power on at `vdd`, as sdhci_set_power and then
    /// sdhci_intel_set_power do it: power register cleared, voltage
    /// selected, power on; then, up to twice, a millisecond later, power on
    /// written again if it did not stick (on Intel hosts the present state
    /// may not have propagated yet after D3 to D0). Fails if it never does.
    pub fn power_up(&mut self, vdd: Vdd) -> EmmcResult<()> {
        let sel = vdd.select();
        self.io.w8(POWER_CONTROL, 0);
        self.io.w8(POWER_CONTROL, sel);
        self.io.w8(POWER_CONTROL, sel | POWER_ON);
        for _ in 0..2 {
            self.pause(1);
            if self.io.r8(POWER_CONTROL) & POWER_ON != 0 {
                break;
            }
            self.io.w8(POWER_CONTROL, sel | POWER_ON);
        }
        let now = self.io.r8(POWER_CONTROL);
        if now & POWER_ON == 0 {
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
