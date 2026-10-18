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

//! The AMD bank's interrupts, levels and outputs: pending pins cleared one
//! by one (write one to the status bit, never to the wake status), then
//! the bank's end of interrupt; an output driven only when the pin's
//! output enable says it is one.

use crate::amd::{AmdBank, W1C};
use crate::amd_bits::*;
use crate::controller::{Controller, GpioError};
use crate::trigger::Trigger;
use crate::window::{update, Window};
use alloc::vec::Vec;

impl<W: Window> Controller for AmdBank<W> {
    fn arm(&mut self, pin: u16, t: Trigger) -> Result<(), GpioError> {
        self.arm_pin(pin, t)
    }

    fn mask(&mut self, pin: u16, masked: bool) -> Result<(), GpioError> {
        self.mask_pin(pin, masked)
    }

    fn take_pending(&mut self, armed: &[u16]) -> Vec<u16> {
        let mut out = Vec::new();
        for &pin in armed {
            let Ok(at) = self.at(pin) else { continue };
            let v = self.window.read(at);
            if v & INTERRUPT_STS != 0 {
                self.window.write(at, (v & !WAKE_STS) | INTERRUPT_STS);
                out.push(pin);
            }
        }
        update(&mut self.window, MASTER, MASTER_EOI, MASTER_EOI);
        out
    }

    fn level(&mut self, pin: u16) -> Result<bool, GpioError> {
        let at = self.at(pin)?;
        Ok(self.window.read(at) & PIN_STS != 0)
    }

    fn drive(&mut self, pin: u16, high: bool) -> Result<(), GpioError> {
        let at = self.at(pin)?;
        let v = self.window.read(at) & !W1C;
        if v & OUTPUT_ENABLE == 0 {
            return Err(GpioError::NotOutput);
        }
        self.window.write(at, if high { v | OUTPUT_VALUE } else { v & !OUTPUT_VALUE });
        Ok(())
    }
}
