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
 * The UEFI variable cache must hold the attribute word firmware returns.
 *
 * The kernel's manager and variable code is included by path and run against a
 * runtime services table whose get_variable reports the specification's words:
 * SecureBoot 0x06 (boot service and runtime access, volatile) and PK 0x27
 * (adds non-volatile and time-based authenticated write). The cache used to
 * store DEFAULT_NV_BS_RT (0x07) for every variable, so both checks below fail
 * against that code.
 */

mod firmware;
mod table;
mod tests;
mod time;
mod unsupported;
