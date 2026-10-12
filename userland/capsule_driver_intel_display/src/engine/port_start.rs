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

//! Bringing a DDI's transmitter up at a link rate and width, and taking
//! it down (SKL PRM Vol 12, DisplayPort enable and disable sequences):
//! the DDI's I/O well, its DPLL set for the rate and locked, its clock
//! routed, then the buffer and the transport with TPS1. A port firmware
//! left without translation entries is refused: their values are the
//! board's and this driver does not invent them.

use super::access::{rmw, Mmio, Until};
use super::ddi::*;
use super::dpll::{enable_reg, DPLL_CTRL1, DPLL_CTRL2, DPLL_STATUS};
use super::dpll_ctrl::{ctrl1_bits, ctrl2_bits};
use super::error::Error;
use super::port::Port;
use super::power::{enable, Well};
use super::wait::wait_for;

const LOCK_US: u64 = 5_000;

impl Port<'_> {
    pub fn bring_up(&mut self, mbps: u32, lanes: u32) -> Result<(), Error> {
        let d = self.index();
        let m: &mut dyn Mmio = &mut *self.mmio;
        if m.read(buf_trans(d, 0)) == 0 {
            return Err(Error::Layout);
        }
        let (mask, bits) = ctrl1_bits(self.dpll, Some(mbps)).ok_or(Error::Argument)?;
        enable(m, Well::DdiIo(d), &Until::new(self.clock, LOCK_US))?;
        if self.dpll == 0 {
            // The boot DPLL runs eDP at the rate firmware chose; train at that.
            if m.read(DPLL_CTRL1) & mask != bits {
                return Err(Error::Argument);
            }
        } else {
            rmw(m, DPLL_CTRL1, mask, bits);
            rmw(m, enable_reg(self.dpll), 1 << 31, 1 << 31);
            let lock = 1 << (self.dpll * 8);
            wait_for(m, DPLL_STATUS, lock, lock, &Until::new(self.clock, LOCK_US))?;
        }
        let (mask, bits) = ctrl2_bits(d, self.dpll);
        rmw(m, DPLL_CTRL2, mask, bits);
        m.write(buf_ctl(d), BUF_ENABLE | width_bits(lanes));
        let enh = if self.enhanced { TP_ENHANCED } else { 0 };
        m.write(tp_ctl(d), TP_ENABLE | enh | TPS1);
        Ok(())
    }
}
