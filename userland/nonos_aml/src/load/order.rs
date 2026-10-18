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

//! Which definition block may load when (section 5.2.11.2).

/// The DSDT first and only first; then SSDTs and PSDTs, and the "OEMx"
/// tables LoadTable may bring in at run time.
pub fn in_order(signature: [u8; 4], first: bool) -> bool {
    match &signature {
        b"DSDT" => first,
        b"SSDT" | b"PSDT" | [b'O', b'E', b'M', _] => !first,
        _ => false,
    }
}
