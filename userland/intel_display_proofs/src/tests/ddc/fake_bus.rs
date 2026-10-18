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

//! A DDC/CI bus with the model monitor on it that notes when each message
//! went and came, on the model clock, without moving the clock itself.

use crate::ddc::bus::DdcBus;
use crate::ddc::error::DdcError;
use crate::model::mccs::Mccs;
use crate::model::Ticks;

pub struct FakeBus<'a> {
    pub clock: &'a Ticks,
    pub monitor: Mccs,
    /// Each send (true) and receive (false), with its time.
    pub log: Vec<(bool, u64)>,
    /// Nothing at the address: every send is NAKed.
    pub absent: bool,
}

impl<'a> FakeBus<'a> {
    pub fn new(clock: &'a Ticks, controls: &[(u8, u16, u16)]) -> FakeBus<'a> {
        FakeBus { clock, monitor: Mccs::with(controls), log: Vec::new(), absent: false }
    }

    pub fn sends(&self) -> Vec<u64> {
        self.log.iter().filter(|l| l.0).map(|l| l.1).collect()
    }
}

impl DdcBus for FakeBus<'_> {
    fn send(&mut self, bytes: &[u8]) -> Result<(), DdcError> {
        self.log.push((true, self.clock.now.get()));
        if self.absent {
            return Err(DdcError::Bus);
        }
        self.monitor.take(bytes);
        Ok(())
    }

    fn receive(&mut self, out: &mut [u8]) -> Result<(), DdcError> {
        self.log.push((false, self.clock.now.get()));
        out.iter_mut().for_each(|b| *b = self.monitor.next_byte());
        Ok(())
    }
}
