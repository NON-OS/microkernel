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

//! Kernel-side glue for the VMware PVSCSI driver capsule: embed and spawn.
//! The driver serves the block wire on `driver.pvscsi0` and answers on
//! endpoint.4294967357, one unit per disk or CD drive its scan found, in
//! target then LUN order; the kernel never touches the adapter.

mod embed;
mod endpoint;
mod spawn;
mod state;

pub use endpoint::ENDPOINT;
pub use spawn::{spawn_driver_pvscsi_capsule, SpawnError};
pub use state::shared_state;
