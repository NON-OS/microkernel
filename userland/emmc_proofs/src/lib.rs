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

//! Host proofs for the eMMC driver (`userland/capsule_driver_ahci/src/emmc`). The
//! driver tree is included file by file with `#[path]`, everything but
//! `platform` (the broker glue), so its `super::` paths resolve unchanged.
//! `model` is a register level SDHCI host with an eMMC device behind it, so
//! the real bring-up, read, write, flush and recovery run end to end.

pub mod emmc;

/// The AHCI capsule's transfer limits, which the eMMC disk shares when the
/// AHCI capsule serves it.
#[path = "../../capsule_driver_ahci/src/constants/ata.rs"]
pub mod ahci_ata;
/// The AHCI capsule's op numbers; the eMMC disk answers the same ops.
#[path = "../../capsule_driver_ahci/src/protocol/ops.rs"]
pub mod ahci_ops;
/// The AHCI capsule's IDENTIFY reply, which names an eMMC part too.
#[path = "../../capsule_driver_ahci/src/protocol/identify_reply.rs"]
pub mod ahci_identify;

#[cfg(test)]
mod model;

#[cfg(test)]
mod adma_tests;
#[cfg(test)]
mod ahci_contract_tests;
#[cfg(test)]
mod bring_up_tests;
#[cfg(test)]
mod clock_tests;
#[cfg(test)]
mod cmd_tests;
#[cfg(test)]
mod fault_tests;
#[cfg(test)]
mod info_tests;
#[cfg(test)]
mod io_tests;
#[cfg(test)]
mod pci_tests;
#[cfg(test)]
mod register_tests;
#[cfg(test)]
mod voltage_tests;
