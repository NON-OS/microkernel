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

//! Who holds the one Linux run, and what it was started for.

use super::roles::RUN;
use crate::services::registry::lookup_service;

/// Whether a store-started or windowed Linux program still holds the run
/// role: its endpoint is released when the process ends, so a second open
/// before then is told it is busy instead of being queued and refused unseen.
pub fn run_busy() -> bool {
    lookup_service(RUN.name).is_some()
}

/// Whether `pid` is the process the kernel registered as the run role,
/// app.linux.run, which no other capsule may claim.
pub fn is_run_role(pid: u32) -> bool {
    lookup_service(RUN.name).is_some_and(|ep| ep.pid == pid)
}
