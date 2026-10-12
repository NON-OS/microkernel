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

//! Every wait, in milliseconds, and where its value comes from.

/// A global reset is held at least 10 ms (UHCI Design Guide, as recalled);
/// held a little longer here.
pub const GRESET_MS: u64 = 15;
/// HCRESET clears itself; allowed this long.
pub const HCRESET_MS: u64 = 50;
/// A root port reset is driven 50 ms (USB 2.0 TDRSTR for a root port, as
/// recalled), then the port is given 10 ms to recover (TRSTRCY).
pub const PORT_RESET_MS: u64 = 50;
pub const PORT_RECOVERY_MS: u64 = 10;
/// PE setting after the reset: allowed this long, polled.
pub const PORT_ENABLE_MS: u64 = 20;
/// SET_ADDRESS gives the device 2 ms to take its address (USB 2.0, as
/// recalled); 10 is allowed.
pub const SET_ADDRESS_MS: u64 = 10;
/// One control transfer: 5 s, the time USB 2.0 allows a request with data.
pub const CONTROL_MS: u64 = 5_000;
/// One bulk transfer of up to 4 KiB.
pub const BULK_MS: u64 = 5_000;
/// The controller halting after RS is cleared: within a frame or two.
pub const HALT_MS: u64 = 10;
