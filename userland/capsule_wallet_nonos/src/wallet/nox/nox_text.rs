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

// A NOX amount in wei as a String, two decimals (format_nox).

use alloc::string::String;

pub fn nox_text(wei: u128) -> String {
    let mut buf = [0u8; 48];
    let n = super::format_nox(wei, &mut buf).unwrap_or(0);
    String::from(core::str::from_utf8(&buf[..n]).unwrap_or("?"))
}
