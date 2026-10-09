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

//! The service lookup: the desktop shell answers by its pid while it is up.

use super::world::with;

pub fn lookup_pid(service: &[u8]) -> Option<u32> {
    if service != b"desktop_shell" {
        return None;
    }
    match with(|world| world.shell) {
        0 => None,
        pid => Some(pid),
    }
}
