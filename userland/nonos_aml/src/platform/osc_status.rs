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

//! The status word _OSC passes both ways (ACPI 6.5, 6.2.11, table 6.21).

use crate::exec::Fault;
use alloc::vec::Vec;

/// Set in the first call: asking what would be granted, changing nothing.
pub const QUERY: u32 = 1 << 0;
const FAILURE: u32 = 1 << 1;
const UNRECOGNIZED_UUID: u32 = 1 << 2;
const UNRECOGNIZED_REVISION: u32 = 1 << 3;
/// Some capability was not granted; informational, not an error.
pub const MASKED: u32 = 1 << 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OscError {
    Fault(Fault),
    /// Not a buffer of at least as many words as were passed.
    Shape,
    Failure,
    UnrecognizedUuid,
    UnrecognizedRevision,
}

pub fn check(status: u32) -> Result<(), OscError> {
    if status & UNRECOGNIZED_UUID != 0 {
        return Err(OscError::UnrecognizedUuid);
    }
    if status & UNRECOGNIZED_REVISION != 0 {
        return Err(OscError::UnrecognizedRevision);
    }
    if status & FAILURE != 0 {
        return Err(OscError::Failure);
    }
    Ok(())
}

/// Firmware's answer cut to what was sent: the status word as it came, and
/// each capability word no wider than the one asked. A bit granted that
/// was not asked for is not taken, so NONOS never holds a control, or
/// claims a capability, it did not ask for; words past those sent are
/// dropped.
pub fn kept(sent: &[u32], got: &[u32]) -> Vec<u32> {
    let cap = |(i, (s, g)): (usize, (&u32, &u32))| if i == 0 { *g } else { s & g };
    sent.iter().zip(got).enumerate().map(cap).collect()
}
