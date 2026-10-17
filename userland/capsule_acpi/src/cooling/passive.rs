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

//! Passive cooling for one zone (ACPI 6.5, 11.1.5): found when the zone has
//! _PSV, believable _TC1, _TC2 and _TSP, and a _PSL naming processors; then
//! stepped every _TSP by `nonos_aml`'s rule, the zone read that often while
//! it is at or above _PSV or still limited. A zone without all of these
//! stays report-only, as every zone was before.

use super::state::Zone;
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use alloc::vec::Vec;
use nonos_aml::cpu::processors;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::thermal::{passive_constants, passive_devices, step, Constants, FULL};

pub struct Passive {
    pub c: Constants,
    /// The ACPI processor UIDs _PSL names.
    pub uids: Vec<u64>,
    /// Percent of full performance.
    pub limit: u8,
    pub last: Option<i32>,
    pub due_ms: u64,
}

/// `limit` carries a zone's limit across a re-read of its objects.
pub fn passive_of(aml: &mut Aml, host: &mut CapsuleHost, tz: Id, limit: u8) -> Option<Passive> {
    let c = passive_constants(aml, &mut evaluator(host), tz)?;
    let psl = passive_devices(aml, &mut evaluator(host), tz);
    let cpus = processors(aml, &mut evaluator(host));
    let uids: Vec<u64> = cpus.iter().filter(|p| psl.contains(&p.id)).map(|p| p.uid).collect();
    (!uids.is_empty()).then_some(Passive { c, uids, limit, last: None, due_ms: 0 })
}

/// After a reading: the next limit when a _TSP has passed, and the zone
/// read again by then while passive cooling is engaged.
pub fn step_zone(z: &mut Zone, now: u64) {
    let (Some(p), Some(psv), Some(t)) = (z.passive.as_mut(), z.trips.passive, z.w.temp) else {
        return;
    };
    // An unread reading moves no limit, since the zone may be hotter than
    // its last, nor reads it sooner: a _TMP cut each time costs a budget.
    if z.w.unread {
        return;
    }
    if now >= p.due_ms {
        p.limit = step(p.limit, p.c, p.last.unwrap_or(t), t, psv);
        (p.last, p.due_ms) = (Some(t), now.saturating_add(p.c.period_ms));
    }
    if t >= psv || p.limit < FULL {
        z.next_ms = z.next_ms.min(p.due_ms);
    }
}
