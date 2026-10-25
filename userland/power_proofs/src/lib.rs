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

//! The wiped power-off from the desktop, on the host: capsule_power's rule
//! for who may power the machine off, the shell's request against the frame
//! the capsule reads, and, from the source, the path from the shell's
//! confirmation to zerostate's terminate.

#[path = "../../capsule_power/src/off/rule.rs"]
pub mod rule;
#[path = "../../capsule_acpi/src/power/off/wire.rs"]
pub mod wire;
#[path = "../../capsule_desktop_shell/src/power_off_client/frame.rs"]
pub mod frame;

#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod kernel_glue_tests;
#[cfg(test)]
mod rule_tests;
