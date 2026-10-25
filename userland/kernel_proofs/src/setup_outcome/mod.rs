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
 * What init reads setup to have asked for. A boot with the static checks
 * running beside it once started the desktop after setup exited 3
 * (install): the status init read had gone. The rules are included by
 * path; glue_tests holds the kernel to calling them.
 */

#[path = "../../../../src/process/exit/kept_table.rs"]
mod kept_table;
#[path = "../../../../src/userspace/capsule_setup_wizard/outcome_rule.rs"]
mod outcome_rule;

#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod tests;
