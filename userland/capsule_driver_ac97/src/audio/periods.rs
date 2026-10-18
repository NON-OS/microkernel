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

/*
 * How each period the controller finished was refilled, counted rather than
 * logged one line at a time. A period the queue filled whole was played. A
 * period it could not fill while nothing was flowing is an idle stream
 * playing silence, which is not a fault. A shortfall after audio was flowing
 * is an underrun, counted once per dry spell however many silent periods
 * follow it, so a client that stops feeding reads as one underrun, not one
 * per period until the stream is stopped.
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Period {
    Played,
    Idle,
    Underrun,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Periods {
    pub played: u32,
    pub underruns: u32,
    flowing: bool,
}

impl Periods {
    pub const fn new() -> Self {
        Periods { played: 0, underruns: 0, flowing: false }
    }

    /// Accounts one refilled period: `filled` bytes of the `want` it holds.
    pub fn period(&mut self, filled: usize, want: usize) -> Period {
        if filled >= want {
            self.played = self.played.saturating_add(1);
            self.flowing = true;
            Period::Played
        } else if self.flowing {
            self.flowing = false;
            self.underruns = self.underruns.saturating_add(1);
            Period::Underrun
        } else {
            Period::Idle
        }
    }

    /// Whether this run played anything, so a stop has something to report.
    pub fn any(&self) -> bool {
        self.played != 0 || self.underruns != 0
    }
}
