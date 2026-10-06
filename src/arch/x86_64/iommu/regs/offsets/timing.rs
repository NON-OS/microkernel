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

/// The spec sets no absolute bound on command completion, so this is a
/// ceiling on a unit that is answering at all: past it the unit is treated as
/// broken rather than waited on forever.
pub const COMMAND_SPINS: u32 = 1_000_000;

/// The same ceiling on the clock, for waits that read uptime. Linux allows
/// a unit ten seconds (DMAR_OPERATION_TIMEOUT); a unit that has not answered
/// in one has stopped, and a stalled boot says so sooner.
pub const COMMAND_MS: u64 = 1000;
