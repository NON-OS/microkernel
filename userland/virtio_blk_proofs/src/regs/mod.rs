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

//! The driver's register accessors, every file as it ships. Port I/O
//! reaches the shim, which hands it to the device model a test attaches
//! (`tests::legacy_dev`), and refuses it when none is.

#[path = "../../../capsule_driver_virtio_blk/src/regs/io.rs"]
mod io;
#[path = "../../../capsule_driver_virtio_blk/src/regs/pio.rs"]
mod pio;
#[path = "../../../capsule_driver_virtio_blk/src/regs/read.rs"]
mod read;
#[path = "../../../capsule_driver_virtio_blk/src/regs/state.rs"]
mod state;
#[path = "../../../capsule_driver_virtio_blk/src/regs/write.rs"]
mod write;

pub use state::Regs;
