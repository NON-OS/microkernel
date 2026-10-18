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

//! The register model a controller's `_HID` picks. The AMD bank is driven
//! in full. An Intel PCH controller is held, but every pin on it is
//! refused until a published layout places its pads (F 46); none is
//! guessed.

use crate::mmio::Mmio;
use alloc::vec::Vec;
use nonos_gpio::amd::AmdBank;
use nonos_gpio::intel::{IntelGpio, IntelPad};
use nonos_gpio::{Controller, GpioError, Trigger};

/// AMD's FCH GPIO `_HID`s, recalled from AMD's published platform
/// descriptions: unverified.
const AMD_IDS: [&[u8; 8]; 2] = [b"AMD0030\0", b"AMDI0030"];

pub enum Ctl {
    Amd(AmdBank<Mmio>),
    Intel(IntelGpio<Mmio>),
}

fn unplaced(_: u16) -> Option<IntelPad> {
    None
}

pub fn ctl(hid: [u8; 8], mut windows: Vec<Mmio>) -> Option<Ctl> {
    if AMD_IDS.iter().any(|id| **id == hid) {
        let w = windows.drain(..).next()?;
        let len = u32::try_from(w.len).ok()?;
        return Some(Ctl::Amd(AmdBank { window: w, len }));
    }
    Some(Ctl::Intel(IntelGpio { communities: Vec::new(), place: unplaced }))
}

impl Ctl {
    fn inner(&mut self) -> &mut dyn Controller {
        match self {
            Ctl::Amd(c) => c,
            Ctl::Intel(c) => c,
        }
    }
}

impl Controller for Ctl {
    fn arm(&mut self, pin: u16, t: Trigger) -> Result<(), GpioError> {
        self.inner().arm(pin, t)
    }
    fn mask(&mut self, pin: u16, masked: bool) -> Result<(), GpioError> {
        self.inner().mask(pin, masked)
    }
    fn take_pending(&mut self, armed: &[u16]) -> Vec<u16> {
        self.inner().take_pending(armed)
    }
    fn level(&mut self, pin: u16) -> Result<bool, GpioError> {
        self.inner().level(pin)
    }
    fn drive(&mut self, pin: u16, high: bool) -> Result<(), GpioError> {
        self.inner().drive(pin, high)
    }
}
