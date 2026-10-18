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

//! The hostile sink's link end: a transmitter that may refuse to start,
//! waits summed so the time a path takes is bounded too, and whether the
//! port is up after a run.

use super::profile::Profile;
use super::rng::Rng;
use super::sink::Hostile;
use crate::dp::lane::Drive;
use crate::dp::link::{Link, Pattern, TrainError};

impl Hostile {
    pub fn new(seed: u64, p: Profile, dpcd: [u8; 16], bound: u32) -> Self {
        let (transfers, status_reads, waited_us, up, starts) = (0, 0, 0, None, 0);
        Hostile {
            rng: Rng::new(seed),
            p,
            dpcd,
            transfers,
            bound,
            status_reads,
            waited_us,
            up,
            starts,
        }
    }
}

impl Link for Hostile {
    fn start(&mut self, mbps: u32, lanes: u32) -> Result<(), TrainError> {
        if self.p.start_fails && self.rng.below(3) == 0 {
            return Err(TrainError::Transmitter);
        }
        (self.up, self.starts) = (Some((mbps, lanes)), self.starts + 1);
        Ok(())
    }

    fn pattern(&mut self, _: Pattern) {}

    fn drive(&mut self, _: Drive) {}

    fn wait_us(&mut self, us: u32) {
        self.waited_us += us as u64;
    }

    fn stop(&mut self) {
        self.up = None;
    }

    fn now_us(&self) -> Option<u64> {
        Some(self.waited_us)
    }
}
