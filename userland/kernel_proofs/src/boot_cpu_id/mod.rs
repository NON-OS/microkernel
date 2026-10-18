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

/*
 * The boot CPU's descriptor takes the id `cpu_id()` will look it up by, and
 * that id has to be the one its local APIC answers to. The kernel's pure rule
 * is included by path.
 *
 * Before it the descriptor took a cache that read 0 that early in boot, so a
 * machine whose boot CPU was not APIC 0 halted at its first `cpu_id()`.
 */

#[path = "../../../../src/arch/x86_64/interrupt/apic/boot_id_rule.rs"]
pub mod boot_id_rule;
mod tests;
