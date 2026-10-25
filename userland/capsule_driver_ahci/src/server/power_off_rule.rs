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

//! The order and the time of the power-off. FLUSH CACHE EXT first, given
//! POWER_OFF_FLUSH_MS; then STANDBY IMMEDIATE, given what the power-off's
//! bound leaves after the flush's longest (constants::power_off), and sent
//! only when that is STANDBY_MIN_MS or more. A restart asks for the flush
//! alone, as a disk spun down only to spin up again gains nothing. Pure, so
//! the host proofs hold it.

use crate::constants::power_off::{standby_budget, POWER_OFF_FLUSH_MS};
use crate::error::{AhciError, AhciResult};

/// How the standby went.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standby {
    Done,
    Failed(AhciError),
    /// A restart: flush alone.
    NotAsked,
    /// Too little of the power-off's bound was left to send it.
    NoTime,
}

/// How the two commands ended.
pub struct PoweredOff {
    pub flushed: AhciResult<()>,
    pub standby: Standby,
}

/// Run `flush` on `port` with its budget, then, when `standby_too`,
/// `standby` with what is left.
pub fn power_off<P, F, S>(port: &mut P, standby_too: bool, flush: F, standby: S) -> PoweredOff
where
    F: FnOnce(&mut P, u64) -> AhciResult<()>,
    S: FnOnce(&mut P, u64) -> AhciResult<()>,
{
    let flushed = flush(port, POWER_OFF_FLUSH_MS);
    let standby = match (standby_too, standby_budget(flushed.is_ok())) {
        (false, _) => Standby::NotAsked,
        (true, None) => Standby::NoTime,
        (true, Some(ms)) => match standby(port, ms) {
            Ok(()) => Standby::Done,
            Err(e) => Standby::Failed(e),
        },
    };
    PoweredOff { flushed, standby }
}

impl PoweredOff {
    /// The first failure, the flush's before the standby's. NoTime comes
    /// only after a failed flush, whose failure is the reply.
    pub fn failure(&self) -> AhciResult<()> {
        self.flushed?;
        match self.standby {
            Standby::Failed(e) => Err(e),
            Standby::Done | Standby::NotAsked | Standby::NoTime => Ok(()),
        }
    }
}
