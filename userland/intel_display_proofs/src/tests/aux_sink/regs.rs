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

//! DDI B's AUX channel registers in front of the sink, from the test's own
//! reading of them: a control write with Send/Busy takes the request from
//! the data registers and answers at once, Done with the reply's size, or
//! Done with the timeout flag when no reply comes. Each transfer moves the
//! clock on as a reply (400 us) or the reply timer (1600 us) would, and a
//! slow try's every transfer on as long as it holds the bus.

use super::sink::AuxSink;
use crate::engine::access::Mmio;
use crate::engine::aux_regs::{aux_ctl, aux_data, Ddi, SEND_BUSY, SIZE_MASK, SIZE_SHIFT};
use crate::model::Ticks;

pub struct AuxRegs<'a> {
    pub clock: &'a Ticks,
    pub sink: AuxSink,
    pub(super) data: [u32; 5],
    pub(super) ctl: u32,
}

impl<'a> AuxRegs<'a> {
    pub fn new(clock: &'a Ticks, sink: AuxSink) -> AuxRegs<'a> {
        AuxRegs { clock, sink, data: [0; 5], ctl: 0 }
    }
}

impl Mmio for AuxRegs<'_> {
    fn read(&mut self, off: u32) -> u32 {
        match (0..5).find(|&i| off == aux_data(Ddi::B, i)) {
            Some(i) => self.data[i as usize],
            None if off == aux_ctl(Ddi::B) => self.ctl,
            None => 0,
        }
    }

    fn write(&mut self, off: u32, v: u32) {
        if let Some(i) = (0..5).find(|&i| off == aux_data(Ddi::B, i)) {
            self.data[i as usize] = v;
        } else if off == aux_ctl(Ddi::B) && v & SEND_BUSY != 0 {
            self.send(((v & SIZE_MASK) >> SIZE_SHIFT) as usize);
        } else if off == aux_ctl(Ddi::B) {
            self.ctl &= !(v & 0x7E00_0000);
        }
    }

    fn write64(&mut self, _: u32, _: u64) {}

    fn read64(&mut self, _: u32) -> u64 {
        0
    }
}
