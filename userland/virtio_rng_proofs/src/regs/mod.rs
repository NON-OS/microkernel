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

//! The driver's register accessors, assembled for the host.
//!
//! `io`, `read`, `state` and `write` are the shipping files. `pio` is the
//! one part that cannot come along: it issues port-IO syscalls through
//! `nonos_libc`, which has no host build. The stand-in below is a legacy
//! device behind a port grant that holds the status byte and its reset, and
//! the request queue's size and page, and panics on any other register, so
//! a test that takes the PIO arm for more is told so instead of reading a
//! zero that would look like a device answering. Tests build every other
//! window with `Regs::mmio`.

#[path = "../../../capsule_driver_virtio_rng/src/regs/io.rs"]
mod io;

mod pio;
mod port;

#[path = "../../../capsule_driver_virtio_rng/src/regs/read.rs"]
mod read;
#[path = "../../../capsule_driver_virtio_rng/src/regs/state.rs"]
mod state;
#[path = "../../../capsule_driver_virtio_rng/src/regs/write.rs"]
mod write;

pub use self::port::{port, Status};
pub use self::state::Regs;
