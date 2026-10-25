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

//! One hot-plug slot, read on each poll: a card is there when presence
//! says so and, on a port that reports it, its link is up. Arrival waits
//! for that to hold `READY_MS`; departure for absence to hold
//! `DEBOUNCE_MS`. The attention button asks for an eject after
//! `BUTTON_MS`, unless pressed again. Pure; the caller does each act.

pub use super::read::{Act, Read};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Empty,
    /// A card is in; since `since`, and steadily there since `steady`.
    Coming { since: u64, steady: Option<u64> },
    Occupied,
    Going { since: u64 },
    Button { until: u64 },
    /// Ejecting, asked again at `next` while a driver holds a function.
    Leaving { next: u64 },
    /// Let go or never up: nothing happens until the card is taken out.
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub phase: Phase,
    pub button: bool,
    pub power: bool,
}

impl Slot {
    /// A slot as found: occupied when a card is up in it.
    pub fn new(button: bool, power: bool, r: Read) -> Slot {
        let phase = if r.here() { Phase::Occupied } else { Phase::Empty };
        Slot { phase, button, power }
    }

    /// The eject took every function out: power goes off where the slot
    /// can switch it, and nothing more happens until the card is out.
    pub fn ejected(&mut self) -> Act {
        self.phase = Phase::Off;
        if self.power { Act::PowerOff } else { Act::Nothing }
    }

    pub fn step(&mut self, r: Read, now: u64) -> Act {
        let fault = super::fault::fault(*self, r);
        let (phase, act) = fault.unwrap_or_else(|| super::step::next(*self, r, now));
        self.phase = phase;
        act
    }
}
