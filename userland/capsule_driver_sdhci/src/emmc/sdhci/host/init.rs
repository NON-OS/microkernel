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

//! Full reset and the polled ADMA2 setup every bring-up starts from.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::text::Line;
use super::super::caps::Caps;
use super::super::regs::{
    CAPABILITIES, CAPABILITIES_1, CLOCK_CONTROL, ERROR_ENABLE, HOST_CONTROL, HOST_CONTROL2,
    HOST_VERSION, INT_ENABLE, INT_STATUS, NORMAL_ENABLE, POWER_CONTROL, RESET_ALL, SIGNAL_ENABLE,
    SPEC_300, TIMEOUT_CONTROL, TIMEOUT_MAX,
};
use super::engine::Host;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Reset the whole host and set it up for polled ADMA2: statuses
    /// latched, no interrupt signalled, longest data timeout, no preset
    /// values, 3.3 V signaling (1.8 V Signaling Enable clear, as Linux
    /// leaves it below HS200), power and clock off, one data line.
    pub fn init(&mut self) -> EmmcResult<()> {
        if let Err(e) = self.reset(RESET_ALL) {
            self.say(Line::new().s(b"host reset did not finish"));
            return Err(e);
        }
        self.caps = Caps {
            caps: self.io.r32(CAPABILITIES),
            caps1: self.io.r32(CAPABILITIES_1),
            version: self.io.r16(HOST_VERSION),
        };
        let c = self.caps;
        self.say_caps();
        if !c.adma2() {
            self.say(Line::new().s(b"host has no ADMA2"));
            return Err(EmmcError::NoAdma);
        }
        self.io.w32(INT_ENABLE, NORMAL_ENABLE as u32 | (ERROR_ENABLE as u32) << 16);
        self.io.w32(SIGNAL_ENABLE, 0);
        self.io.w32(INT_STATUS, u32::MAX);
        self.io.w8(TIMEOUT_CONTROL, TIMEOUT_MAX);
        self.io.w8(POWER_CONTROL, 0);
        self.io.w16(CLOCK_CONTROL, 0);
        self.io.w8(HOST_CONTROL, 0);
        if c.spec() >= SPEC_300 {
            self.io.w16(HOST_CONTROL2, 0);
        }
        self.vdd = None;
        self.sd_hz = 0;
        self.width = 1;
        self.hs = false;
        self.detect();
        Ok(())
    }
}
