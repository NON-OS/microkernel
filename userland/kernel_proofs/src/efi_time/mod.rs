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
 * EFI times follow the calendar and count back before 1970.
 *
 * The kernel's EfiTime comes from the UEFI modules the crate already includes
 * by path. is_valid bounded the day by 31 in every month, so February 31
 * passed and converted as March 3, and to_unix_timestamp only counted years
 * forward from 1970, so every date from 1900 to 1969 converted as a date in
 * 1970. The checks below fail against that code.
 */

mod tests;
