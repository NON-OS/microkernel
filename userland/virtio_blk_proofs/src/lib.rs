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

//! The virtio-blk driver, proved against register windows, rings and
//! buffers in host memory rather than against a device.
//!
//! Each `#[path]` below pulls in the shipping driver source, so these tests
//! run the code that boots, bring-up included. Its kernel calls resolve to
//! `libc_shim`: a scripted interrupt line, a clock that moves only when a
//! wait takes time, a device list, a broker that grants and logs, and a
//! legacy device on ports that the tests model (`tests::legacy_dev`). The
//! server loop, which receives over IPC, is held in its source text
//! (`tests::order_tests`, `tests::rules_tests`); what it does after each
//! request (`server::settle`) runs here, the bring-up it redoes included.
//!
//! What is held: the request parsers (exact framing, every request on the
//! disk); the request wait, ended by the clock and never by a count of
//! wakes; a request the device never answers stalling the queue rather
//! than every later request; the used element taken only when it names
//! the request in flight with the length that request has; the chain and
//! ring laid out as the specification lays them; the legacy handshake;
//! discovery; the orders bring-up and teardown keep; and a run of resets
//! that ends with the disk given up.

extern crate alloc;

#[path = "../../capsule_driver_virtio_blk/src/clock/mod.rs"]
pub mod clock;
#[path = "../../capsule_driver_virtio_blk/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_virtio_blk/src/discover/mod.rs"]
pub mod discover;
#[path = "../../capsule_driver_virtio_blk/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_virtio_blk/src/io/mod.rs"]
pub mod io;
#[path = "../../capsule_driver_virtio_blk/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_virtio_blk/src/queue/mod.rs"]
pub mod queue;
pub mod regs;
pub mod server;
pub mod setup;
#[path = "../../capsule_driver_virtio_blk/src/transport/mod.rs"]
pub mod transport;
#[path = "../../capsule_driver_virtio_blk/src/vectors.rs"]
pub mod vectors;

#[cfg(test)]
mod capacity_tests;
#[cfg(test)]
mod tests;

#[cfg(kani)]
mod kani;
