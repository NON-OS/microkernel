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

//! ACPI fixed hardware: FADT decoding, Generic Address Structure access,
//! sleep entry and reset. `fadt_decode`, `gas`, `sleep`, `reset`, `power_button` and
//! `madt_cpu` are pure and proven on the host (acpi_aml_proofs); `port_bus`
//! is the kernel's bus behind them.

pub mod fadt_decode;
pub mod gas;
pub mod madt_cpu;
pub mod port_bus;
pub mod power_button;
pub mod reset;
pub mod sleep;
