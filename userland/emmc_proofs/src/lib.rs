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

//! Host proofs for the eMMC driver (`userland/capsule_driver_sdhci/src/emmc`). The
//! driver tree is included file by file with `#[path]`, everything but
//! `platform` (the broker glue), so its `super::` paths resolve unchanged.
//! `model` is a register level SDHCI host with an eMMC device behind it, so
//! the real bring-up, read, write, flush and recovery run end to end.

extern crate alloc;

pub mod emmc;

/// The kernel's list of the Intel eMMC hosts it starts the capsule for.
#[path = "../../../src/hardware/inventory/emmc.rs"]
pub mod kernel_inventory;
pub mod kernel_wire;
/// Who the capsule serves the medium to.
#[path = "../../capsule_driver_sdhci/src/server/medium_rule.rs"]
pub mod medium_rule;
/// The capsule's block wire, held to the kernel client's in `kernel_wire`.
#[path = "../../capsule_driver_sdhci/src/protocol/mod.rs"]
pub mod protocol;
/// How the capsule checks the head of a READ or WRITE.
#[path = "../../capsule_driver_sdhci/src/server/handlers/rw_parse.rs"]
pub mod rw_parse;

#[cfg(test)]
mod model;

#[cfg(test)]
mod adma_tests;
#[cfg(test)]
mod bring_up_tests;
#[cfg(test)]
mod card_tests;
#[cfg(test)]
mod clock_tests;
#[cfg(test)]
mod cmd_tests;
#[cfg(test)]
mod console_tests;
#[cfg(test)]
mod deadline_tests;
#[cfg(test)]
mod fault_tests;
#[cfg(test)]
mod io_tests;
#[cfg(test)]
mod names_tests;
#[cfg(test)]
mod pci_tests;
#[cfg(test)]
mod reason_tests;
#[cfg(test)]
mod register_tests;
#[cfg(test)]
mod start_list_tests;
#[cfg(test)]
mod voltage_tests;
#[cfg(test)]
mod wire_server_tests;
