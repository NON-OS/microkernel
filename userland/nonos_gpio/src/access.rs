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

//! Output pins and stormed pins: an output is bound to the one device
//! whose GpioIo names it and driven only for a sender holding that device;
//! a pin masked for storming is unmasked only when its owner asks.

use crate::controller::{Controller, GpioError};
use crate::router::{BindError, Router};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputError {
    /// No device's GpioIo names the pin, or the sender holds another.
    NotYours,
    Gpio(GpioError),
}

impl Router {
    /// An output pin `device`'s GpioIo names; another device's is refused.
    pub fn bind_output(&mut self, pin: u16, device: u64) -> Result<(), BindError> {
        match self.outputs.iter().find(|(p, _)| *p == pin) {
            Some((_, d)) if *d != device => Err(BindError::Taken),
            Some(_) => Ok(()),
            None => {
                self.outputs.push((pin, device));
                Ok(())
            }
        }
    }

    /// Drive `pin` for a sender holding the devices `holds` says it does.
    pub fn output(
        &self,
        ctl: &mut impl Controller,
        holds: impl Fn(u64) -> bool,
        pin: u16,
        high: bool,
    ) -> Result<(), OutputError> {
        let device = self.outputs.iter().find(|(p, _)| *p == pin).map(|(_, d)| *d);
        match device {
            Some(d) if holds(d) => ctl.drive(pin, high).map_err(OutputError::Gpio),
            _ => Err(OutputError::NotYours),
        }
    }

    /// Unmask a pin that stormed, at its owner's request.
    pub fn rearm(&mut self, ctl: &mut impl Controller, pin: u16) -> Result<(), GpioError> {
        let b = self.bindings.iter_mut().find(|b| b.pin == pin).ok_or(GpioError::Unplaced)?;
        ctl.mask(pin, false)?;
        (b.masked, b.storm) = (false, Default::default());
        Ok(())
    }
}
