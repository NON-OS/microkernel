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

//! The source's side of training: the transmitter a GPU driver hands the
//! trainer, the patterns it sends, and what a trained link is.

use super::aux::AuxError;
use super::lane::Drive;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// Normal operation: video, scrambled.
    Off,
    Tps1,
    Tps2,
    Tps3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrainError {
    Aux(AuxError),
    /// Every rate and width tried failed a phase.
    Failed,
    /// The transmitter could not run at a rate the sink offered.
    Transmitter,
}

impl From<AuxError> for TrainError {
    fn from(e: AuxError) -> Self {
        TrainError::Aux(e)
    }
}

/// The GPU's half: its port and PLL, its pattern generator, its drive.
pub trait Link {
    /// Bring the port up at `mbps` per lane on `lanes` lanes.
    fn start(&mut self, mbps: u32, lanes: u32) -> Result<(), TrainError>;
    fn pattern(&mut self, p: Pattern);
    fn drive(&mut self, d: Drive);
    /// Sleep at least `us` microseconds.
    fn wait_us(&mut self, us: u32);
    /// Take the port down after a failed attempt.
    fn stop(&mut self);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Trained {
    pub mbps: u32,
    pub lanes: u32,
    pub drive: Drive,
}
