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

//! The AMD bank as a `Controller`: one dword per pin, `amd_bits` laid on
//! it. A pin is placed when its dword lies inside the window and is not
//! the master register's. Arming and masking here; interrupts, levels and
//! outputs in `amd_io`.

use crate::amd_bits::*;
use crate::controller::GpioError;
use crate::trigger::{Polarity, Trigger};
use crate::window::Window;

pub struct AmdBank<W: Window> {
    pub window: W,
    /// The window's length in bytes, from the controller's `_CRS`.
    pub len: u32,
}

/// Status bits a read-modify-write must not write back as ones.
pub(crate) const W1C: u32 = INTERRUPT_STS | WAKE_STS;

impl<W: Window> AmdBank<W> {
    pub(crate) fn at(&self, pin: u16) -> Result<u32, GpioError> {
        let off = u32::from(pin) * 4;
        (off + 4 <= self.len && off != MASTER).then_some(off).ok_or(GpioError::Unplaced)
    }

    pub(crate) fn arm_pin(&mut self, pin: u16, t: Trigger) -> Result<(), GpioError> {
        let at = self.at(pin)?;
        let level = match t.polarity {
            Polarity::High => 0,
            Polarity::Low => 1,
            Polarity::Both => 2,
        };
        let bits = if t.edge { 0 } else { LEVEL_TRIG } | level << ACTIVE_LEVEL_SHIFT;
        let mask = LEVEL_TRIG | ACTIVE_LEVEL_MASK | INTERRUPT_ENABLE | INTERRUPT_DELIVER;
        let v = self.window.read(at) & !W1C;
        self.window.write(at, (v & !mask) | bits | INTERRUPT_ENABLE | INTERRUPT_DELIVER);
        Ok(())
    }

    pub(crate) fn mask_pin(&mut self, pin: u16, masked: bool) -> Result<(), GpioError> {
        let at = self.at(pin)?;
        let v = self.window.read(at) & !W1C;
        let v = if masked { v & !INTERRUPT_DELIVER } else { v | INTERRUPT_DELIVER };
        self.window.write(at, v);
        Ok(())
    }
}
