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

//! The battery and AC state the ACPI capsule last read from firmware, for
//! MkBatteryStatus. One word, written whole, so a reader never sees half a
//! report; with the time it was written, so a capsule that stopped
//! reporting stops being believed.

use core::sync::atomic::{AtomicU64, Ordering};

/// Bits 0-7 the charge in percent, 0xFF for unknown.
pub const PERCENT_MASK: u64 = 0xFF;
pub const BATTERY_PRESENT: u64 = 1 << 8;
pub const CHARGING: u64 = 1 << 9;
pub const DISCHARGING: u64 = 1 << 10;
pub const AC_ONLINE: u64 = 1 << 11;
/// Set when the platform has an AC adapter whose state was read.
pub const AC_KNOWN: u64 = 1 << 12;
pub const KNOWN_BITS: u64 = (1 << 13) - 1;

/// The capsule reads every minute; three missed readings and the report
/// is stale.
const FRESH_MS: u64 = 180_000;

static REPORT: AtomicU64 = AtomicU64::new(0);
static WRITTEN_MS: AtomicU64 = AtomicU64::new(0);

pub fn record(word: u64, now_ms: u64) {
    REPORT.store(word, Ordering::Release);
    WRITTEN_MS.store(now_ms.max(1), Ordering::Release);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Battery {
    /// No fresh report: the capsule is not running, or not yet.
    Unreported,
    Absent,
    /// Present, charge unknown.
    Present,
    Percent(u8),
}

pub fn battery(now_ms: u64) -> Battery {
    let at = WRITTEN_MS.load(Ordering::Acquire);
    if at == 0 || now_ms.saturating_sub(at) > FRESH_MS {
        return Battery::Unreported;
    }
    let w = REPORT.load(Ordering::Acquire);
    match (w & BATTERY_PRESENT != 0, w & PERCENT_MASK) {
        (false, _) => Battery::Absent,
        (true, p) if p <= 100 => Battery::Percent(p as u8),
        (true, _) => Battery::Present,
    }
}
