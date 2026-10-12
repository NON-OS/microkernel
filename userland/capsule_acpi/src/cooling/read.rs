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

//! Reading a zone's temperature and judging it. _CRT is believed only on
//! two readings at least a second apart, since one stray _TMP must not
//! power the machine off; a zone at or past _CRT is read every second
//! until that is settled, and every five while it lasts.

use super::state::Zone;
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use nonos_aml::load::Aml;
use nonos_aml::thermal::{judge, temperature};

/// For a zone with no _TZP, or a _TZP of 0: notifications should come, but
/// a missed one must not hide a zone heating towards _CRT.
const DEFAULT_POLL_MS: u64 = 30_000;
const MIN_POLL_MS: u64 = 1_000;
const MAX_POLL_MS: u64 = 60_000;
const CONFIRM_MS: u64 = 1_000;
const CRITICAL_POLL_MS: u64 = 5_000;

fn interval(z: &Zone) -> u64 {
    match z.trips.polling {
        Some(ds) if ds > 0 => ds.saturating_mul(100).clamp(MIN_POLL_MS, MAX_POLL_MS),
        _ => DEFAULT_POLL_MS,
    }
}

pub fn read_zone(z: &mut Zone, aml: &mut Aml, host: &mut CapsuleHost, now: u64) {
    z.temp = temperature(aml, &mut evaluator(host), z.id);
    let v = z.temp.map(|t| judge(&z.trips, t, &z.verdict.active)).unwrap_or_default();
    z.critical = match (v.critical, z.critical_at) {
        (false, _) => {
            z.critical_at = None;
            false
        }
        (true, None) => {
            z.critical_at = Some(now);
            false
        }
        (true, Some(at)) => z.critical || now.saturating_sub(at) >= CONFIRM_MS,
    };
    let wait = match (v.critical, z.critical) {
        (true, false) => CONFIRM_MS,
        (true, true) => CRITICAL_POLL_MS,
        _ => interval(z),
    };
    z.next_ms = now.saturating_add(wait);
    z.verdict = v;
}
