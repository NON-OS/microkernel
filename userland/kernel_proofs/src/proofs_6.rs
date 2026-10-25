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

//! Host proofs, group 6 of 8: test modules mounted by path
//! from proofs.rs, each at its own file.

#[path = "acpi_span/mod.rs"]
pub mod acpi_span;
/// The GOP present refused while a claimed device holds the boot framebuffer.
#[path = "boot_fb/mod.rs"]
mod boot_fb;
#[path = "block_wire/mod.rs"]
mod block_wire;
/// Disk controllers before the bus drivers.
#[path = "disks_first/mod.rs"]
mod disks_first;
#[path = "described/mod.rs"]
pub mod described;
/// The firmware's flash never claimed or mapped.
#[path = "firmware_flash/mod.rs"]
mod firmware_flash;
#[path = "firmware_message/mod.rs"]
pub mod firmware_message;
#[path = "inventory_ids/mod.rs"]
pub(crate) mod inventory_ids;
/// The power-off's leave phase: every disk driver asked, each bounded.
#[path = "leave_rule/mod.rs"]
mod leave_rule;
/// The memory type a MkMmioMap mapping gets: write-combined only where allowed.
#[path = "mmio_cache/mod.rs"]
mod mmio_cache;
/// terminate()'s once: one caller to the wipe, however many race.
#[path = "terminate_once/mod.rs"]
mod terminate_once;
#[path = "unit_base/mod.rs"]
pub mod unit_base;
#[path = "secret_log/mod.rs"]
pub mod secret_log;
/// When a lock wait is reported, and called a re-take.
#[path = "spin_watch/mod.rs"]
pub mod spin_waits;
/// H-06(b): one surface's cap, by the boot framebuffer.
#[path = "surface_cap/mod.rs"]
pub mod surface_cap;
#[path = "pio_reset/mod.rs"]
pub mod pio_reset;
#[path = "panic_plan/mod.rs"]
pub mod panic_plan;
#[path = "panic_stage/mod.rs"]
pub mod panic_stage;
#[path = "proof_io_spawn_tests.rs"]
#[cfg(test)]
mod proof_io_spawn_tests;
#[path = "proof_io_spawn_walk.rs"]
#[cfg(test)]
mod proof_io_spawn_walk;
/// A user page never GLOBAL, now that CR4.PGE is on (P-06).
#[path = "user_not_global/mod.rs"]
pub mod user_not_global;
