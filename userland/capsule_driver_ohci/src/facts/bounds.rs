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

//! Every bound a wait has. Values from the specification are as recalled
//! and unverified; the rest are this engine's own.

/// Firmware's SMM driver given the controller after an ownership change
/// request (this engine's own, generous: SMM handlers run at their pace).
pub const OWNERSHIP_MS: u64 = 500;
/// A host reset clears within 10 us (spec); 10 ms is this engine's bound.
pub const HCR_MS: u64 = 10;
/// The root hub times a port reset itself, about 10 ms (spec); reset done
/// is waited for this long.
pub const PORT_RESET_MS: u64 = 50;
/// After reset, before the device is spoken to (USB 2.0 TRSTRCY, 10 ms).
pub const PORT_RECOVERY_MS: u64 = 10;
/// Power-on to power-good when the root hub does not say (this engine's).
pub const POWER_MIN_MS: u64 = 20;
/// SET_ADDRESS's recovery (USB 2.0: 2 ms; this engine waits longer).
pub const SET_ADDRESS_MS: u64 = 10;
/// A control transfer and a bulk transfer of up to 4 KiB.
pub const CONTROL_MS: u64 = 5000;
pub const BULK_MS: u64 = 5000;
/// A frame to pass once an ED is skipped, before its words are rewritten.
pub const FRAME_MS: u64 = 5;
