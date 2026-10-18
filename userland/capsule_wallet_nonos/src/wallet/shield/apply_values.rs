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

//! A job answer's named values, as text and as numbers.

use alloc::string::String;

use shield_wire::*;

pub(super) fn text(body: &str, name: &str) -> String {
    String::from(field(body, name).unwrap_or(""))
}
pub(super) fn number(body: &str, name: &str) -> Option<u32> {
    field(body, name)?.parse().ok()
}
