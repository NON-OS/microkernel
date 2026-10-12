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

//! The thermal word for the kernel: the hottest zone's temperature in
//! tenths of a kelvin, and every trip point any zone has reached. Sent when
//! it changes, at least once a minute so the kernel keeps believing it, and
//! at every reading while a zone is critical, so the kernel sees how long.

use super::state::{Cooling, Zone};
use crate::sys::acpi_event::{acpi_event, CRITICAL, HOT, PASSIVE, TENTHS_K_MASK, THERMAL};
use crate::sys::log::Line;
use nonos_aml::thermal::tenths_kelvin;

const RESEND_MS: u64 = 60_000;
const FLAGS: u64 = PASSIVE | HOT | CRITICAL;

pub fn word(c: &Cooling) -> Option<u64> {
    let hottest = c.zones.iter().filter_map(|z| z.temp).max()?;
    let any = |f: fn(&Zone) -> bool, bit: u64| if c.zones.iter().any(f) { bit } else { 0 };
    let flags = any(|z| z.verdict.passive, PASSIVE)
        | any(|z| z.verdict.hot, HOT)
        | any(|z| z.critical, CRITICAL);
    Some(tenths_kelvin(hottest).min(TENTHS_K_MASK) | flags)
}

fn say(w: u64) {
    let dk = w & TENTHS_K_MASK;
    let (c10, neg) = if dk >= 2732 { (dk - 2732, false) } else { (2732 - dk, true) };
    let l = Line::new(b"thermal hottest ").str(if neg { b"-" } else { b"" });
    let l = l.dec(c10 / 10).str(b".").dec(c10 % 10).str(b" C");
    let l = if w & PASSIVE != 0 { l.str(b" passive") } else { l };
    let l = if w & HOT != 0 { l.str(b" hot") } else { l };
    if w & CRITICAL != 0 { l.str(b" CRITICAL") } else { l }.end();
}

pub fn send(c: &mut Cooling, now: u64) {
    let Some(w) = word(c) else { return };
    let due = now.saturating_sub(c.reported_ms) >= RESEND_MS;
    if c.reported == Some(w) && !due && w & CRITICAL == 0 {
        return;
    }
    if c.reported.map(|r| r & FLAGS) != Some(w & FLAGS) {
        say(w);
    }
    // A refused report is made again at the next reading.
    if acpi_event(THERMAL, w).is_ok() {
        (c.reported, c.reported_ms) = (Some(w), now);
    }
}
