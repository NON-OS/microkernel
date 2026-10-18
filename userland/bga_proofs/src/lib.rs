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

//! The Bochs display adapter's driver, proved against a register window
//! and a framebuffer in memory rather than against an adapter.
//!
//! Each `#[path]` below pulls in the shipping driver source, so these tests
//! run the code that boots, not a copy of it: the DISPI mode set, the mode
//! kept from firmware and written back, and the display wire it serves the
//! compositor. Bochs, QEMU and every clone of them implement the same
//! published register set, so a driver that programs the wrong bits per
//! pixel, or leaves the linear framebuffer undecoded, is wrong against all
//! of them: the class of defect a test with no adapter can pin down.

#[path = "../../capsule_driver_bga/src/constants.rs"]
pub mod constants;
pub mod dispi;
#[path = "../../capsule_driver_bga/src/error.rs"]
pub mod error;
/// The shipping register access, each write answered by the adapter model.
pub mod regs;
/// virtio-gpu's op numbers and lengths, which the display wire must match;
/// the rest of its wire is not served here.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../capsule_driver_virtio_gpu/src/protocol/limits.rs"]
mod virtio_limits;
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../capsule_driver_virtio_gpu/src/protocol/ops.rs"]
mod virtio_ops;

/// One bring-up attempt end to end, against `libc_shim`'s broker and kernel:
/// discovery's record, the claim and PCI command, both BAR mappings, the
/// DISPI ID, the mode kept or set and read back, the clear and the surface.
#[cfg(test)]
#[path = "../../capsule_driver_bga/src/discover.rs"]
mod discover;
#[cfg(test)]
#[path = "../../capsule_driver_bga/src/handles.rs"]
mod handles;
/// The display wire the driver serves once it is up.
#[cfg(test)]
mod serve;
#[cfg(test)]
#[path = "../../capsule_driver_bga/src/setup/mod.rs"]
mod setup;

#[cfg(test)]
mod adapter;
#[cfg(test)]
mod check_tests;
#[cfg(test)]
mod clear_tests;
#[cfg(test)]
mod display_tests;
#[cfg(test)]
mod endian_tests;
#[cfg(test)]
mod mode_tests;
#[cfg(test)]
mod offset_tests;
#[cfg(test)]
mod reason_tests;
#[cfg(test)]
mod setup_bound_tests;
#[cfg(test)]
mod setup_claim_tests;
#[cfg(test)]
mod setup_confirm_tests;
#[cfg(test)]
mod setup_tests;
#[cfg(test)]
mod span_tests;
#[cfg(test)]
mod unblank_tests;
#[cfg(test)]
mod virtio_wire_tests;
