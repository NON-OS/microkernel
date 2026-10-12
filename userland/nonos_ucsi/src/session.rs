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

//! A running PPM session: notifications admitted at a bounded rate, the
//! connectors they name read a few per tick, a stuck PPM reset and walked
//! again with growing pauses, and given up after `MAX_RESETS` in a row.

use super::exchange::Fault;
use super::layout::Cci;
use super::poll::note;
use super::ppm::Ppm;
use super::start::{start, Port};
use super::storm::Storm;
use alloc::vec::Vec;

pub const PER_TICK: usize = 4;
pub const MAX_RESETS: u8 = 3;
pub const BACKOFF_MS: u64 = 5000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Running,
    Recovering { failures: u8, at: u64 },
    Failed,
}

pub struct Session {
    pub ports: Vec<Port>,
    pub pending: u128,
    pub state: State,
    pub storm: Storm,
    /// Connectors whose status changed since the caller last took them.
    pub changed: u128,
}

impl Session {
    pub fn start(ppm: &mut impl Ppm) -> Result<Self, Fault> {
        let mut pending = 0;
        let ports = start(ppm, &mut pending)?;
        let (state, storm, changed) = (State::Running, Storm::default(), 0);
        Ok(Session { ports, pending, state, storm, changed })
    }

    /// A Notify on the device: the CCI read, at the storm's rate.
    pub fn notified(&mut self, ppm: &mut impl Ppm, now: u64) {
        if self.state != State::Running || !self.storm.admit(now) {
            return;
        }
        if let Some(v) = ppm.signal_read().then(|| ppm.read_cci()).flatten() {
            note(&mut self.pending, &Cci::decode(v));
        }
    }

    pub fn tick(&mut self, ppm: &mut impl Ppm, now: u64) {
        match self.state {
            State::Running => self.serve(ppm, now),
            State::Recovering { failures, at } if now >= at => self.recover(ppm, now, failures),
            _ => {}
        }
    }
}
