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

//! Which device each pin belongs to, from the firmware's GpioInt and
//! GpioIo descriptors (capsule_acpi's SERIAL records, its `_AEI` pins):
//! an interrupt pin is armed with its descriptor's trigger and bound to
//! one owner; an output pin to the one device whose descriptor names it.
//! A pin bound to one owner is refused to another.

use crate::controller::{Controller, GpioError};
use crate::storm::Storm;
use crate::trigger::Trigger;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    /// The device whose `_CRS` names the pin, by the key its holder is
    /// known by (the capsule uses its bus controller's PCI function).
    Device(u64),
    /// An `_AEI` pin: capsule_acpi runs the event method.
    Aei,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindError {
    Taken,
    Gpio(GpioError),
}

pub(crate) struct Binding {
    pub pin: u16,
    pub owner: Owner,
    pub masked: bool,
    pub storm: Storm,
}

#[derive(Default)]
pub struct Router {
    pub(crate) bindings: Vec<Binding>,
    pub(crate) outputs: Vec<(u16, u64)>,
}

impl Router {
    pub fn bind(
        &mut self,
        ctl: &mut impl Controller,
        pin: u16,
        owner: Owner,
        t: Trigger,
    ) -> Result<(), BindError> {
        if let Some(b) = self.bindings.iter().find(|b| b.pin == pin) {
            return if b.owner == owner { Ok(()) } else { Err(BindError::Taken) };
        }
        ctl.arm(pin, t).map_err(BindError::Gpio)?;
        self.bindings.push(Binding { pin, owner, masked: false, storm: Storm::default() });
        Ok(())
    }

    pub fn owner(&self, pin: u16) -> Option<Owner> {
        self.bindings.iter().find(|b| b.pin == pin).map(|b| b.owner)
    }
}
