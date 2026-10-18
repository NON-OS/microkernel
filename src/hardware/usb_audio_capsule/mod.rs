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

//! Kernel-side glue for the USB audio class capsule: embed and spawn it.
//! audio.server reaches it by name (`driver.usb_audio0`) on the NHDA wire,
//! held to the server alone (`services/registry/held_table.rs`), and it
//! reaches its device through driver.xhci0.

mod embed;
mod spawn;
mod state;

pub use spawn::{spawn_driver_usb_audio_capsule, SpawnError};
pub use state::shared_state;
