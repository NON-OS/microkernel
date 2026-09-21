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

//! The half of a relay that only a microdescriptor carries.

/// What a microdescriptor contributes to a usable relay.
#[derive(Clone, Default)]
pub struct Microdesc {
    /// B, the key an ntor handshake to this relay is against.
    pub ntor_onion_key: [u8; 32],
    /// The identity an EXTEND2 pins the hop to.
    pub ed25519_identity: [u8; 32],
    /// Whether the published exit summary lets a web port out.
    pub exits_web: bool,
}
