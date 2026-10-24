// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The leave phase, before power goes (zerostate terminate): every running
//! disk driver asked at once to make its disks safe, NVMe and AHCI with
//! their power-off op (the controller's shutdown; the disk's flush, then
//! its standby), the rest with a flush. Each is bounded by its own budget and
//! all of them by the total (rule.rs), so a disk that never answers costs
//! its budget and the power-off goes on. One line per driver says how it
//! ended. An amnesic boot flushes too: the drive's own cache must settle.
//! A restart flushes AHCI disks without spinning them down (run.rs).

mod leg;
mod leg_poll;
mod poll;
mod rule;
mod run;

pub(crate) use leg::{Leg, Wire};
pub(crate) use rule::{CRITICAL_TOTAL_MS, TOTAL_MS};
pub(crate) use run::power_off_disks;
