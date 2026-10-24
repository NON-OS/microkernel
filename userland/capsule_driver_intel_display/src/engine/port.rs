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

//! One DDI as a DisplayPort source: its AUX channel and its transmitter
//! (DPLL, buffer and transport) behind the trainer's `Source`.

use super::access::{Clock, Mmio};
use super::aux_ch::AuxCh;
use super::aux_regs::Ddi;
use crate::dp::aux::{Aux, AuxError};

pub struct Port<'a> {
    pub mmio: &'a mut dyn Mmio,
    pub ddi: Ddi,
    /// The DPLL this DDI runs on; 0 is the boot DPLL, which an eDP port
    /// shares and which is never reprogrammed.
    pub dpll: u32,
    pub enhanced: bool,
    pub clock: &'a dyn Clock,
}

impl Port<'_> {
    pub fn index(&self) -> u32 {
        self.ddi as u32
    }
}

impl Aux for Port<'_> {
    fn transfer(&mut self, req: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError> {
        AuxCh { mmio: &mut *self.mmio, ddi: self.ddi, clock: self.clock }.transfer(req, reply)
    }
}
