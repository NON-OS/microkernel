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

//! The kernel's own virtio-rng (L8): read for boot entropy, through rings
//! in real DMA frames, until `capsule_driver_virtio_rng` claims the device
//! and the kernel hands it over.

mod api;
mod device;
mod handover;
mod init;
mod queue;
mod state;

pub use api::{fill_random, get_random_bytes, is_available};
pub use handover::hand_over;
pub use init::init as init_virtio_rng;
