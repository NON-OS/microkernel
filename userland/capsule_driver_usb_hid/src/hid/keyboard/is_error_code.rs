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

/// ErrorRollOver (0x01), POSTFail (0x02) and ErrorUndefined (0x03). A
/// keyboard fills its key slots with one of these when it cannot say which
/// keys are down, most often because more are down than a report holds.
pub(in crate::hid::keyboard) fn is_error_code(key: u8) -> bool {
    matches!(key, 0x01..=0x03)
}
