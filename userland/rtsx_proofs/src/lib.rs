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

//! Host-runnable proofs for capsule_driver_rtsx. The driver's pure modules
//! are included by path and checked against the values Linux's rtsx_pcr.c,
//! rtsx_pci_sdmmc.c and mmc core compute, and the SD specification's field
//! layouts.

#[path = "../../capsule_driver_rtsx/src/regs/mod.rs"]
pub mod regs;

#[path = "../../capsule_driver_rtsx/src/chip/mod.rs"]
pub mod chip;

#[path = "../../capsule_driver_rtsx/src/wire/mod.rs"]
pub mod wire;

#[path = "../../capsule_driver_rtsx/src/sd/mod.rs"]
pub mod sd;

#[path = "../../capsule_driver_rtsx/src/clock/budget.rs"]
pub mod budget;

#[path = "../../capsule_driver_rtsx/src/log/line.rs"]
pub mod line;

#[cfg(test)]
mod chip_tests;
#[cfg(test)]
mod clock_tests;
#[cfg(test)]
mod csd_tests;
#[cfg(test)]
mod encode_tests;
#[cfg(test)]
mod fields;
#[cfg(test)]
mod line_tests;
#[cfg(test)]
mod response_tests;
#[cfg(test)]
mod sd_tests;
