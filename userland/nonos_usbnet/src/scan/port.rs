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

//! A root port as `OP_PORT_STATUS` reports it.

/// A root port with a device connected, which class driver holds it
/// (`OP_PORT_STATUS` owner byte: 0 free, 2 claimed), and whether its
/// connection changed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Port {
    pub id: u8,
    pub owner: u8,
    pub changed: bool,
}
