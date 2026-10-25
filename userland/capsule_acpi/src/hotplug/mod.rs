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

//! ACPI hot-plug and docking: devices that come and go while the system
//! runs. A device that arrives is probed into the kernel's table; one that
//! leaves is taken out, and the kernel revokes what its holder was given
//! before it answers; an eject waits for every holder to let go.

mod arrive;
mod cover;
mod dock;
mod eject;
mod functions;
mod leave;
mod look;
pub mod native;
mod notify;
mod pending;
mod records;
mod release;
mod remember;
mod stage;
mod state;
mod take_out;
mod verify;

pub use notify::notified;
pub use pending::tick;
pub use remember::learn;
pub use state::Hotplug;
