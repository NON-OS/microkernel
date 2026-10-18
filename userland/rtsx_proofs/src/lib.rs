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

//! Host-runnable proofs for capsule_driver_rtsx. The driver's modules are
//! included by path: the pure ones are checked against the values Linux's
//! rtsx_pcr.c, rtsx_pci_sdmmc.c and mmc core compute and the SD
//! specification's field layouts; the engine, card, setup and block-service
//! flows run over `nonos_libc` (libc_shim: a virtual clock, the broker, DMA
//! memory, IPC) and a modelled reader behind the register type (`hw`,
//! `model`).

extern crate alloc;

#[path = "../../capsule_driver_rtsx/src/card/mod.rs"]
pub mod card;
#[path = "../../capsule_driver_rtsx/src/chip/mod.rs"]
pub mod chip;
#[path = "../../capsule_driver_rtsx/src/clock/mod.rs"]
pub mod clock;
#[path = "../../capsule_driver_rtsx/src/engine/mod.rs"]
pub mod engine;
#[path = "../../capsule_driver_rtsx/src/error/mod.rs"]
pub mod error;
pub mod hw;
#[path = "../../capsule_driver_rtsx/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_rtsx/src/log/mod.rs"]
pub mod log;
#[path = "../../capsule_driver_rtsx/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_rtsx/src/regs/mod.rs"]
pub mod regs;
#[path = "../../capsule_driver_rtsx/src/report.rs"]
pub mod report;
#[path = "../../capsule_driver_rtsx/src/sd/mod.rs"]
pub mod sd;
#[path = "../../capsule_driver_rtsx/src/serve/mod.rs"]
pub mod serve;
#[path = "../../capsule_driver_rtsx/src/setup/mod.rs"]
pub mod setup;
#[path = "../../capsule_driver_rtsx/src/wire/mod.rs"]
pub mod wire;

pub mod fields;
pub mod model;

/// nonos_blk_client's driver table and identify parse, held to this
/// driver's wire so the two cannot drift apart.
#[cfg(test)]
#[path = "../../nonos_blk_client/src/driver/mod.rs"]
pub mod blk_driver;
#[cfg(test)]
#[path = "../../nonos_blk_client/src/device/identity_parse.rs"]
pub mod blk_identity;
#[cfg(test)]
#[path = "../../nonos_blk_client/src/status_text.rs"]
pub mod blk_status;

#[cfg(test)]
mod tests;
