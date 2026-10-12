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

//! How long the driver waits on the IOC.

/// A message unit reset, and the IOC leaving the reset state.
pub const RESET_MS: u64 = 10_000;
/// Each doorbell step of a handshake.
pub const DOORBELL_MS: u64 = 1_000;
/// PORT ENABLE: the IOC brings its port up.
pub const PORT_ENABLE_MS: u64 = 10_000;
/// A request. Shorter than the client's own bound, so the driver is the one
/// that says why a request failed.
pub const REQUEST_MS: u64 = 30_000;
/// Looks taken with a yield between them before a wait starts sleeping a
/// millisecond between looks.
pub const QUICK_LOOKS: u32 = 64;
