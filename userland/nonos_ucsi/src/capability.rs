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

//! What GET_CAPABILITY and GET_CONNECTOR_CAPABILITY answer, the fields read
//! here (UCSI 1.2; offsets recalled, unverified).

/// GET_CAPABILITY: bmAttributes (4 bytes), then bNumConnectors, whose
/// seven low bits count the connectors.
pub fn connectors(b: &[u8]) -> Option<u8> {
    b.get(4).map(|n| n & 0x7F)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConnectorCapability {
    /// Dual-role data (bmOperationMode bit 2).
    pub drp: bool,
    /// Can source power (byte 1 bit 0).
    pub provider: bool,
    /// Can sink power (byte 1 bit 1).
    pub consumer: bool,
}

pub fn connector_capability(b: &[u8]) -> Option<ConnectorCapability> {
    let (mode, power) = (*b.first()?, *b.get(1)?);
    Some(ConnectorCapability {
        drp: mode & 0b100 != 0,
        provider: power & 0b01 != 0,
        consumer: power & 0b10 != 0,
    })
}
