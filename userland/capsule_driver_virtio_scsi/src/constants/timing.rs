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

//! How long the driver waits on the device. A request answers at once on an
//! idle host; this bounds one the host stalls, shorter than the client's own
//! bound so the driver is the one that says why a request failed.

pub const COMMAND_MS: u64 = 30_000;
/// Looks taken with a yield between them before the wait starts sleeping a
/// millisecond between looks.
pub const QUICK_LOOKS: u32 = 64;
