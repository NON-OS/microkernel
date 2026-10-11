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

//! The Bochs adapter driver, proved against stand-ins rather than against a
//! display adapter: a register window in memory, and a model of the DISPI
//! registers that records every access in order.
//!
//! Each `#[path]` below pulls in the shipping driver source, so these tests
//! run the code that boots, not a copy of it. Bochs, QEMU and every clone
//! implement the same published register set, so a mode set in the wrong
//! order, a rect copied past its row, or a present from the wrong process is
//! wrong against all of them, and is caught here without a boot.

#[path = "../../capsule_driver_bga/src/constants.rs"]
pub mod constants;
pub mod dispi;
#[path = "../../capsule_driver_bga/src/error.rs"]
pub mod error;
pub mod gfx;
#[path = "../../capsule_driver_bga/src/hw.rs"]
pub mod hw;
pub mod model;
#[path = "../../capsule_driver_bga/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_bga/src/regs.rs"]
pub mod regs;
#[path = "../../capsule_driver_virtio_gpu/src/protocol/limits.rs"]
pub mod virtio_limits;
#[path = "../../capsule_driver_virtio_gpu/src/protocol/ops.rs"]
pub mod virtio_ops;

#[cfg(test)]
mod choose_tests;
#[cfg(test)]
mod clear_tests;
#[cfg(test)]
mod copy_tests;
#[cfg(test)]
mod mode_tests;
#[cfg(test)]
mod offset_tests;
#[cfg(test)]
mod read_tests;
#[cfg(test)]
mod reason_tests;
#[cfg(test)]
mod screen_tests;
#[cfg(test)]
mod window_tests;
#[cfg(test)]
mod wire_tests;
