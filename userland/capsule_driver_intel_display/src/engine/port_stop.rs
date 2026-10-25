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

//! Taking a DDI's transmitter down after a link failed or on hand-back.

use super::access::{rmw, Mmio, Until};
use super::ddi::{buf_ctl, tp_ctl, BUF_IDLE};
use super::dpll::{enable_reg, DPLL_CTRL2};
use super::error::Error;
use super::port::Port;
use super::wait::wait_for;

const IDLE_US: u64 = 5_000;

impl Port<'_> {
    /// Transport and buffer off, the buffer seen idle, the DDI's clock
    /// off, and its DPLL off unless it is the boot one.
    pub fn take_down(&mut self) -> Result<(), Error> {
        let d = self.index();
        let m: &mut dyn Mmio = &mut *self.mmio;
        m.write(tp_ctl(d), 0);
        m.write(buf_ctl(d), 0);
        wait_for(m, buf_ctl(d), BUF_IDLE, BUF_IDLE, &Until::new(self.clock, IDLE_US))?;
        rmw(m, DPLL_CTRL2, 1 << (15 + d), 1 << (15 + d));
        if self.dpll != 0 {
            rmw(m, enable_reg(self.dpll), 1 << 31, 0);
        }
        Ok(())
    }
}
