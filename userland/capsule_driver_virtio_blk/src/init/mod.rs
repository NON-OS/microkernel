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

//! The virtio handshake on the legacy register window, split where the DMA
//! grants and bus mastering go in (`setup::legacy`): `negotiate`, then
//! `queue_size`, `program`, `capacity` and `driver_ok`. A step that does
//! not hold marks the device FAILED; its caller then resets it.

mod config;
mod negotiate;
mod queue;

pub use config::{capacity, driver_ok};
pub use negotiate::negotiate;
pub use queue::{program, queue_size};
