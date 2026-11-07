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

//! The ACPI capsule's records where there is no ACPI: aarch64 and riscv64
//! boards describe themselves in a device tree, so no capsule ever reports.
//! The boot census and init read the same three records as on x86_64, each
//! the same file, and each stays as it starts: no power word, the lid not
//! known, the first pass not done.

#[path = "acpi/first_pass.rs"]
pub mod first_pass;
#[path = "acpi/lid_state.rs"]
pub mod lid_state;
#[path = "acpi/power_report.rs"]
pub mod power_report;
