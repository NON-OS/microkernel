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
//! Only a Generic Desktop pointer or mouse is fed to the tablet decoder; a
//! security key, a gamepad or a vendor channel is left to its own driver.

use super::application::application;

const GENERIC_DESKTOP: u16 = 0x01;
const POINTER: u16 = 0x01;
const MOUSE: u16 = 0x02;

/// Whether the tablet decoder may read this interface's reports.
pub fn is_pointer(report: &[u8]) -> bool {
    matches!(application(report), Some((GENERIC_DESKTOP, POINTER | MOUSE)))
}
