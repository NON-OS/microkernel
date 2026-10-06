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

//! Which TSC rate a CPU records, kept free of CPUID and port access so the
//! host proofs (clock_resolve_proofs) can check the order.
//!
//! The boot CPU settles the rate before any AP is started: CPUID on Intel,
//! then the PIT, then the ACPI PM timer. Every core counts the same TSC, so
//! an AP takes that rate. It used to measure again from CPUID leaf 0x15 and
//! 0x16, then the PIT. AMD enumerates neither leaf, so every AMD machine
//! reached the PIT poll on its first AP, a poll with no timeout: on a board
//! with its PIT gated that AP never came back, and each AP started after it
//! entered the same poll and reprogrammed PIT channel 0 under the others.

/// The settled rate when there is one; otherwise an enumerated rate; the
/// measurement runs only when neither exists.
pub(crate) fn pick_tsc_hz(
    settled: u64,
    enumerated: impl FnOnce() -> Option<u64>,
    measured: impl FnOnce() -> u64,
) -> u64 {
    if settled != 0 {
        return settled;
    }
    match enumerated() {
        Some(hz) => hz,
        None => measured(),
    }
}
