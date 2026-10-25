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
//! L55, the lead's half: init's wait for the ACPI capsule's first publish
//! pass. Each look is the kernel's own rule (first_pass_rule.rs, mounted at
//! crate::hardware::broker::acpi); the glue pins where init asks it, and that
//! its bound covers the capsule's own reveal budget.

#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod look_tests;
