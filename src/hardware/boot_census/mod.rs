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

//! The boot census: every device class the platform reports (the ACPI
//! tables, the PCI and platform device table, the firmware framebuffer)
//! against what came up, each missing one with its reason, and one summary
//! line the boot matrix and `tools/nonos-bringup-gate` check
//!.

mod gather;
pub mod rule;
mod run;
pub mod when;

pub use run::{start, text, tick};
pub use when::whole_lines;
pub use when::WAIT_MS;
