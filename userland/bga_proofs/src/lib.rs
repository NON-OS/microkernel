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

//! The Bochs DISPI mode set, proved against a register window rather than
//! against a display adapter.
//!
//! Each `#[path]` below pulls in the shipping driver source, so these tests
//! run the code that boots, not a copy of it. The BGA capsule had no host
//! coverage before this crate: the only way to learn that a mode set was
//! wrong was to boot a machine and look at a black screen, which says nothing
//! about which of the five register writes was at fault.
//!
//! The interface is worth proving without hardware because Bochs, QEMU and
//! every clone of them implement the same published register set. A driver
//! that programs the wrong bits-per-pixel, or leaves the linear framebuffer
//! undecoded, is wrong against all of them, and a black screen is the same
//! symptom for each. That is exactly the class of defect a test with no
//! adapter can pin down.

/*
 * The framebuffer clear takes a raw user virtual address, which the driver
 * has no way to describe in a safety comment beyond what its own module says.
 * That capsule has its own clippy job where its style is judged; re-judging it
 * from inside the proof crate would only make the two disagree.
 */
#![allow(clippy::missing_safety_doc)]

#[path = "../../capsule_driver_bga/src/constants.rs"]
pub mod constants;

#[path = "../../capsule_driver_bga/src/error.rs"]
pub mod error;

#[path = "../../capsule_driver_bga/src/regs.rs"]
pub mod regs;

#[path = "../../capsule_driver_bga/src/dispi/mod.rs"]
pub mod dispi;

/*
 * `dispi_off` is `pub(super)` in the capsule, so the copy reached through
 * `dispi` above is sealed inside it. Including the same file a second time at
 * the crate root puts the identical arithmetic where a test can call it. The
 * lib target itself never calls it, hence the allow.
 */
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "../../capsule_driver_bga/src/dispi/dispi_off.rs"]
pub mod dispi_offsets;

#[cfg(test)]
mod clear_tests;

#[cfg(test)]
mod mode_tests;

#[cfg(test)]
mod offset_tests;
