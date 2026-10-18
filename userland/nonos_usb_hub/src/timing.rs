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

//! The waits of bringing up the device on a hub port (USB 2.0 sections
//! 7.1.7.3, 7.1.7.5 and 11.5.1.5, as recalled, unverified), each a deadline
//! the port is looked at against, never a sleep.

/// TATTDB: how long a connection must hold before the port is reset.
pub const DEBOUNCE_MS: u64 = 100;
/// The port is first read this long after its reset is asked for. A hub
/// may not show the reset begun at once, and a port enabled before it would
/// then look reset already (as observed, unverified).
pub const SHORT_RESET_MS: u64 = 10;
/// A reset still running after this has failed (as observed, unverified).
pub const RESET_TIMEOUT_MS: u64 = 800;
/// TRSTRCY: the device's time after its reset before its first request.
pub const RECOVERY_MS: u64 = 10;
