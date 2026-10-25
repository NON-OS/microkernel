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

//! F-107: what the kernel writes when the ACPI capsule leaves, as the
//! kernel builds it: every event enable cleared, the Global Lock's word
//! released and GBL_RLS when firmware waits.

// Mounted once, beside the FADT decoder they build on.
pub(crate) use crate::seeded_tables::{fadt_decode, gas, leave_events};

#[cfg(test)]
mod bus;
#[cfg(test)]
mod disarm_gen;
#[cfg(test)]
mod disarm_tests;
#[cfg(test)]
mod lock_tests;
