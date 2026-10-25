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

//! The time one training may take, every rate and width together. Each
//! AUX try is bounded by the transmitter's wait and each phase by its try
//! count, but a sink that answers only on its last retry, every time,
//! could still hold a mode set for minutes; past this the training ends as
//! an AUX timeout and the output is lost. Our bound, not the standard's.

use super::aux::{Aux, AuxError};
use super::lane::Drive;
use super::link::{Link, Pattern, Source, TrainError};

pub const TRAIN_US: u64 = 2_000_000;

/// The source, refusing every AUX transfer once the time is spent; a
/// source with no clock has none to spend.
pub struct Budget<'a> {
    pub s: &'a mut dyn Source,
    pub start: Option<u64>,
}

impl Aux for Budget<'_> {
    fn transfer(&mut self, req: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError> {
        let spent = match (self.start, self.s.now_us()) {
            (Some(a), Some(n)) => n.saturating_sub(a),
            _ => u64::MAX,
        };
        if spent > TRAIN_US {
            return Err(AuxError::Timeout);
        }
        self.s.transfer(req, reply)
    }
}

impl Link for Budget<'_> {
    fn start(&mut self, mbps: u32, lanes: u32) -> Result<(), TrainError> {
        self.s.start(mbps, lanes)
    }
    fn pattern(&mut self, p: Pattern) {
        self.s.pattern(p)
    }
    fn drive(&mut self, d: Drive) {
        self.s.drive(d)
    }
    fn wait_us(&mut self, us: u32) {
        self.s.wait_us(us)
    }
    fn stop(&mut self) {
        self.s.stop()
    }
    fn now_us(&self) -> Option<u64> {
        self.s.now_us()
    }
}
