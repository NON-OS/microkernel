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

//! An EC that stops answering or will not stop asking. Three waits in a row
//! that time out mean it has stalled; more events than the storm bound in a
//! window mean it floods. Either way the caller turns it off: its space is
//! refused and its GPE quieted, so one bad EC cannot hold the capsule.
//! Three is our bound, from no source.
//!
//! A stall at boot is often an EC still starting, or one firmware's SMI
//! handler was driving, and off for the
//! whole boot would lose the battery, AC, lid and thermal readings with it.
//! So a stalled EC is tried again after a back-off, 1 s doubling to 32 s,
//! at most `TRIALS` times; on trial a single timeout turns it off again, so
//! a dead EC costs one second per trial, about six in all. A flooding EC
//! stays off. Whatever the pattern, `BUDGET` timed-out waits in a boot (each
//! about a second) turn it off for good, so an EC that answers just often
//! enough to keep clearing its strikes cannot hold the capsule either.

use super::regs::EcError;

pub const STRIKES: u8 = 3;
/// Trials after a stall; then the EC stays off for the boot.
pub const TRIALS: u8 = 6;
pub const FIRST_BACKOFF_MS: u64 = 1_000;
/// Timed-out waits in a boot, in all, before the EC is off for good: about
/// twenty seconds of the capsule's time. Our bound, from no source.
pub const BUDGET: u16 = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Off {
    Stalled,
    Flooded,
    /// `BUDGET` timeouts spent this boot.
    Spent,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Health {
    strikes: u8,
    pub off: Option<Off>,
    /// When a stall was first seen off by `retry`, and the trials spent.
    off_seen: Option<u64>,
    trials: u8,
    timeouts: u16,
}

impl Health {
    /// Timeouts in a row so far.
    pub fn strikes(&self) -> u8 {
        self.strikes
    }

    pub fn usable(&self) -> bool {
        self.off.is_none()
    }

    /// After one access: a timeout is a strike and any answer clears them;
    /// a port the kernel refused is neither. True when this turned it off.
    pub fn note<T>(&mut self, r: &Result<T, EcError>) -> bool {
        match r {
            Err(EcError::Timeout) => {
                self.strikes = self.strikes.saturating_add(1);
                self.timeouts = self.timeouts.saturating_add(1);
            }
            Ok(_) => self.strikes = 0,
            Err(EcError::Io) => {}
        }
        if self.timeouts >= BUDGET {
            return self.spend();
        }
        self.turn_off(self.strikes >= STRIKES, Off::Stalled)
    }

    /// Off for good: a stall already off becomes spent, so no trial follows.
    fn spend(&mut self) -> bool {
        let newly = self.off != Some(Off::Spent);
        self.off = Some(Off::Spent);
        newly
    }

    /// The EC raised past the storm bound. True when this turned it off.
    pub fn flooded(&mut self) -> bool {
        self.turn_off(true, Off::Flooded)
    }

    /// Called on the capsule's loop: true when a stalled EC's back-off has
    /// passed and it is on trial again, its space served and its GPE to be
    /// armed again by the caller.
    pub fn retry(&mut self, now_ms: u64) -> bool {
        if self.off != Some(Off::Stalled) || self.trials >= TRIALS {
            return false;
        }
        let seen = *self.off_seen.get_or_insert(now_ms);
        let wait = FIRST_BACKOFF_MS << self.trials;
        if now_ms.saturating_sub(seen) < wait {
            return false;
        }
        self.trials += 1;
        (self.off, self.off_seen, self.strikes) = (None, None, STRIKES - 1);
        true
    }

    /// Timed-out waits since boot.
    pub fn timeouts(&self) -> u16 {
        self.timeouts
    }

    /// Trials spent since boot.
    pub fn trials(&self) -> u8 {
        self.trials
    }

    fn turn_off(&mut self, now: bool, why: Off) -> bool {
        let newly = now && self.off.is_none();
        if newly {
            self.off = Some(why);
        }
        newly
    }
}
