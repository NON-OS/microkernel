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

//! Card detect on a slot with no switch wired to it.

use super::super::super::env::{Clock, Deadline, Log, Mmio};
use super::super::super::text::Line;
use super::super::regs::{
    HC_CD_TEST_LEVEL, HC_CD_TEST_SELECT, HOST_CONTROL, PRESENT_STATE, PS_CARD_INSERTED,
    PS_CARD_STABLE, SPEC_200,
};
use super::engine::Host;
use super::limits::DETECT_MS;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// An embedded slot has no card detect switch, and a host that reads
    /// one as empty may refuse bus power or commands. Once the detect
    /// logic has settled, a slot that still reads empty is told it holds a
    /// card through the Card Detect Test Level (SDHCI 2.00 and later).
    pub(super) fn detect(&self) {
        let d = Deadline::after(&self.clock, DETECT_MS);
        while self.io.r32(PRESENT_STATE) & PS_CARD_STABLE == 0 && !d.passed(&self.clock) {
            self.clock.relax();
        }
        let ps = self.io.r32(PRESENT_STATE);
        if ps & PS_CARD_INSERTED != 0 || self.caps.spec() < SPEC_200 {
            return;
        }
        let hc = self.io.r8(HOST_CONTROL) | HC_CD_TEST_LEVEL | HC_CD_TEST_SELECT;
        self.io.w8(HOST_CONTROL, hc);
        self.say(
            Line::new()
                .s(b"card detect read empty (present ")
                .hex(ps as u64)
                .s(b"), forced present"),
        );
    }
}
