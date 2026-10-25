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

//! What a zone's temperature calls for (ACPI 6.5, 11.1): each active level
//! is on at or above its _ACx, and stays on until the zone has cooled a
//! little below it, so a zone resting on a trip point does not switch its
//! fan at every reading.

use super::trips::Trips;
use alloc::vec::Vec;

pub const HYSTERESIS_MC: i32 = 2_000;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Verdict {
    pub critical: bool,
    pub hot: bool,
    pub passive: bool,
    /// Whether each _ALx's devices should run, _AL0 first.
    pub active: Vec<bool>,
}

pub fn judge(t: &Trips, temp: i32, was: &[bool]) -> Verdict {
    let reached = |trip: Option<i32>| trip.is_some_and(|c| temp >= c);
    let on = |(x, ac): (usize, &i32)| {
        temp >= *ac || (was.get(x).copied().unwrap_or(false) && temp > *ac - HYSTERESIS_MC)
    };
    Verdict {
        critical: reached(t.critical),
        hot: reached(t.hot),
        passive: reached(t.passive),
        active: t.active.iter().enumerate().map(on).collect(),
    }
}
