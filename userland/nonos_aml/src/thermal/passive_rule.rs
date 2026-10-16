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

//! Passive cooling's rule (ACPI 6.5, 11.1.5, recalled, unverified): every
//! _TSP tenths of a second the zone's processors' performance moves by
//! ΔP[%] = _TC1 × (Tn − Tn−1) + _TC2 × (Tn − Tt), where Tn and Tn−1 are this
//! reading and the last and Tt is _PSV. The temperatures are taken in whole
//! kelvin; that the formula means kelvin and not tenths is recalled too.
//! Pure: the limit is a percentage of full performance, which only falls
//! while the zone is at or above _PSV and below it climbs back to 100 by at
//! least a point a step. NONOS's frequency control reads it as a ceiling.

pub const FULL: u8 = 100;
/// Our choice, from no source: larger than any constant firmware writes,
/// so a value past it is taken for a lie.
pub const MAX_TC: u64 = 64;
/// Our choice, from no source, to catch a lie: five minutes, in tenths of
/// a second.
pub const MAX_TSP: u64 = 3000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Constants {
    pub tc1: i64,
    pub tc2: i64,
    pub period_ms: u64,
}

/// None for a constant absent or past its bound, a _TSP of 0, or two zero
/// constants, which would never move the limit: the zone stays report-only.
pub fn constants(tc1: Option<u64>, tc2: Option<u64>, tsp: Option<u64>) -> Option<Constants> {
    let tc = |v: Option<u64>| v.filter(|v| *v <= MAX_TC).map(|v| v as i64);
    let (tc1, tc2) = (tc(tc1)?, tc(tc2)?);
    let tsp = tsp.filter(|t| (1..=MAX_TSP).contains(t))?;
    (tc1 + tc2 > 0).then_some(Constants { tc1, tc2, period_ms: tsp * 100 })
}

/// The next limit from the last, with temperatures in m°C.
pub fn step(limit: u8, c: Constants, last: i32, now: i32, psv: i32) -> u8 {
    let (rise, over) = (i64::from(now) - i64::from(last), i64::from(now) - i64::from(psv));
    let dp = (c.tc1 * rise + c.tc2 * over) / 1000;
    let limit = i64::from(limit); // past 100 is clamped with the rest
    let next = if over >= 0 { limit - dp } else { limit + (-dp).max(1) };
    next.clamp(0, i64::from(FULL)) as u8
}
