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

//! Every bound a wait has. Values from the specifications are as recalled
//! and unverified; the rest are this engine's own.

/// Firmware letting the controller go (this engine's own).
pub const HANDOFF_MS: u64 = 1000;
/// A running controller halts within 16 micro-frames (spec).
pub const HALT_MS: u64 = 10;
/// A host reset clears (this engine's own).
pub const HCRESET_MS: u64 = 250;
/// The schedules report on, and an async advance comes, within a frame or
/// two; these bounds are this engine's own.
pub const SCHEDULE_MS: u64 = 10;
pub const DOORBELL_MS: u64 = 10;
/// A root port held in reset (USB 2.0 TDRST, 10 ms; 50 ms for a root port),
/// then its reset seen to end (spec: within 2 ms).
pub const PORT_RESET_MS: u64 = 50;
pub const PORT_RESET_END_MS: u64 = 5;
/// After reset, before the device is spoken to (USB 2.0 TRSTRCY, 10 ms).
pub const PORT_RECOVERY_MS: u64 = 10;
/// Port power on to good (this engine's own).
pub const POWER_MS: u64 = 20;
pub const SET_ADDRESS_MS: u64 = 10;
pub const CONTROL_MS: u64 = 5000;
pub const BULK_MS: u64 = 5000;
