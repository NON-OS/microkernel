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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyType {
    Symmetric = 0,
    PrivateKey = 1,
    PublicKey = 2,
    HmacSecret = 3,
    DerivedKey = 4,
    SessionKey = 5,
    MasterKey = 6,
    SigningKey = 7,
    Secp256k1Eth = 8,
    /// An HD wallet's recovery words, kept beside its account key for the
    /// shield: the eth key's id (4, little endian), the count, the indices.
    ShieldSeed = 9,
    /// A Bluetooth bond (L33): the peer's identity, LTK and IRK, 48 octets
    /// in the Bluetooth capsule's own layout. Kept for a service name.
    BtBond = 10,
    /// This host's own Bluetooth identity resolving key, 17 octets.
    BtLocalIrk = 11,
}

impl KeyType {
    /// Kinds their holder's restart must not lose: owned by the service name
    /// that stored them, never reaped with a pid (`store/named.rs`).
    pub fn held_by_name(self) -> bool {
        matches!(self, Self::BtBond | Self::BtLocalIrk)
    }
}
