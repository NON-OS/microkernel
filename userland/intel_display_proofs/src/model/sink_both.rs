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

//! The sink's two ends as one source object, as a trainer takes them.

use super::sink::Sink;
use super::sink_aux::AuxEnd;
use super::sink_link::LinkEnd;
use crate::dp::aux::{Aux, AuxError};
use crate::dp::lane::Drive;
use crate::dp::train::{Link, Pattern, TrainError};

/// Both ends of the source as one object, as a trainer takes them.
pub struct Both(pub Sink);

impl Aux for Both {
    fn transfer(&mut self, req: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError> {
        AuxEnd(self.0.clone()).transfer(req, reply)
    }
}

impl Link for Both {
    fn start(&mut self, mbps: u32, lanes: u32) -> Result<(), TrainError> {
        LinkEnd(self.0.clone()).start(mbps, lanes)
    }
    fn pattern(&mut self, p: Pattern) {
        LinkEnd(self.0.clone()).pattern(p)
    }
    fn drive(&mut self, d: Drive) {
        LinkEnd(self.0.clone()).drive(d)
    }
    fn wait_us(&mut self, us: u32) {
        LinkEnd(self.0.clone()).wait_us(us)
    }
    fn stop(&mut self) {
        LinkEnd(self.0.clone()).stop()
    }
    fn now_us(&self) -> Option<u64> {
        LinkEnd(self.0.clone()).now_us()
    }
}
