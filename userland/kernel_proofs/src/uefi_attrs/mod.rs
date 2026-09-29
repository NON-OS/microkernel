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
 * Every authenticated-write flag makes a variable need authentication.
 *
 * The kernel's variable attribute type comes from the UEFI modules the crate
 * already includes by path. requires_authentication counted the time-based and
 * enhanced flags but not the older count-based AUTHENTICATED_WRITE_ACCESS,
 * which firmware still reports on variables written before UEFI 2.3.1, so a
 * set carrying only that flag was answered as needing none. The check below
 * fails against that code.
 */

mod tests;
