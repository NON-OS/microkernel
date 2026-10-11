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

//! The ACPI namespace device scopes (type 5) a DRHD lists. Each names an
//! ANDD by its number and gives the requester id the device's DMA arrives
//! under, as a start bus and (device, function) hops. Pure.

use super::dmar_scope::MAX_PATH;

pub const SCOPE_NAMESPACE: u8 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NamespaceScope {
    pub number: u8,
    pub start_bus: u8,
    pub path: [(u8, u8); MAX_PATH],
    pub path_len: u8,
}

impl NamespaceScope {
    pub const EMPTY: NamespaceScope =
        NamespaceScope { number: 0, start_bus: 0, path: [(0, 0); MAX_PATH], path_len: 0 };

    /// The requester id, when the path is one hop from its start bus. A
    /// longer path runs through bridges whose bus numbers only enumeration
    /// knows; it is not guessed, and the device stays unconfined.
    pub const fn requester(&self) -> Option<(u8, u8, u8)> {
        let (device, function) = self.path[0];
        if self.path_len == 1 && device < 32 && function < 8 {
            Some((self.start_bus, device, function))
        } else {
            None
        }
    }
}
