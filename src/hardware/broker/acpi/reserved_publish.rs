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

//! The motherboard resource ranges the ACPI capsule read from the
//! namespace, kept by the broker, and MCFG's ECAM windows checked against
//! them: configuration space firmware did not reserve is space a BAR or
//! another grant could be placed over, which the owner should know.

use super::super::reserved::{add, decode_ranges, kept_cover, MEMORY};
use super::ecam_note::ecam_reserved;

#[derive(Debug, PartialEq, Eq)]
pub enum ReservedError {
    Malformed,
    /// More ranges than the broker keeps.
    Full,
}

/// Answers how many ranges the broker keeps in all.
pub fn publish(bytes: &[u8]) -> Result<usize, ReservedError> {
    let ranges = decode_ranges(bytes).ok_or(ReservedError::Malformed)?;
    let kept = add(&ranges).ok_or(ReservedError::Full)?;
    let unreserved = super::kernel::platform::fw::pcie_segments().iter().any(|s| {
        let first = u64::from(s.start_bus) << 20;
        let len = (u64::from(s.end_bus) + 1 - u64::from(s.start_bus)) << 20;
        !kept_cover(MEMORY, s.base_address.saturating_add(first), len)
    });
    ecam_reserved(!unreserved);
    Ok(kept)
}
