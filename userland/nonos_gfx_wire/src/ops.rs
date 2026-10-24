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

//! The ops (section 12). 0x02-0x0F are each driver's own; virtio-gpu
//! keeps 0x02-0x0C, and 0x08, 0x09, 0x0A and 0x0C are also the path a
//! compositor uses with a driver that answers CAPS with ENOSYS.

pub const HEALTHCHECK: u16 = 0x01;
pub const PRIVATE_FIRST: u16 = 0x02;
pub const PRIVATE_LAST: u16 = 0x0F;
pub const CAPS: u16 = 0x10;
pub const MODES: u16 = 0x11;
pub const STATE: u16 = 0x12;
pub const SURFACE: u16 = 0x13;
pub const FLUSH: u16 = 0x14;
pub const MODE_SET: u16 = 0x15;
pub const RETIRE: u16 = 0x16;
pub const CURSOR: u16 = 0x17;

/// The milliseconds a MODE_SET may take before the compositor gives up.
pub const MODE_SET_BUDGET_MS: u64 = 1000;
