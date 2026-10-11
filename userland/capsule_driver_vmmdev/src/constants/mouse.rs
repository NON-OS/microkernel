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

//! Events and mouse features.

/// New mouse position data is waiting: the only event this driver asks for.
pub const EVENT_MOUSE_POSITION: u32 = 1 << 9;

/// The guest takes absolute positions and reads them on an event.
pub const GUEST_CAN_ABSOLUTE: u32 = 1 << 0;
pub const NEW_PROTOCOL: u32 = 1 << 4;
/// The host's integration is on: the pointer moves with the host's.
pub const HOST_WANTS_ABSOLUTE: u32 = 1 << 1;

/// Both axes run from 0 to 0xFFFF over the whole guest display.
pub const RANGE_MAX: i32 = 0xFFFF;

/// How long the loop sleeps when no event is pending.
pub const POLL_MS: u64 = 4;
