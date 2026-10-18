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

//! Intel communities' interrupts, levels and outputs: a pending pin is one
//! whose group status bit is set while its enable bit is, cleared by
//! writing that bit alone; the level is the pad's RX state; an output is
//! driven only when the pad's TX buffer is enabled.

use crate::controller::{Controller, GpioError};
use crate::intel::IntelGpio;
use crate::intel_bits::*;
use crate::trigger::Trigger;
use crate::window::Window;
use alloc::vec::Vec;

impl<W: Window> Controller for IntelGpio<W> {
    fn arm(&mut self, pin: u16, t: Trigger) -> Result<(), GpioError> {
        self.arm_pin(pin, t)
    }

    fn mask(&mut self, pin: u16, masked: bool) -> Result<(), GpioError> {
        self.mask_pin(pin, masked)
    }

    fn take_pending(&mut self, armed: &[u16]) -> Vec<u16> {
        let mut out = Vec::new();
        for &pin in armed {
            let Some(p) = (self.place)(pin) else { continue };
            let Some(c) = self.communities.get_mut(p.community) else { continue };
            let (is, ie) = (c.gpi_is + 4 * u32::from(p.group), c.gpi_ie + 4 * u32::from(p.group));
            let bit = 1u32 << p.bit;
            if c.window.read(is) & c.window.read(ie) & bit != 0 {
                c.window.write(is, bit);
                out.push(pin);
            }
        }
        out
    }

    fn level(&mut self, pin: u16) -> Result<bool, GpioError> {
        let (c, at, _) = self.dw0(pin)?;
        Ok(c.window.read(at) & RX_STATE != 0)
    }

    fn drive(&mut self, pin: u16, high: bool) -> Result<(), GpioError> {
        let (c, at, _) = self.dw0(pin)?;
        let v = c.window.read(at);
        if v & TX_DISABLE != 0 || v & PMODE_MASK != 0 {
            return Err(GpioError::NotOutput);
        }
        c.window.write(at, if high { v | TX_STATE } else { v & !TX_STATE });
        Ok(())
    }
}
