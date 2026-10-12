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

//! The record a finished pairing leaves. The peer is kept by the identity
//! it distributed (Vol 3, Part H, 3.6.5); a peer that sent none is kept by
//! the address it connected from, if that address is an identity itself.
//! One that connected from a private address and sent no IRK could never
//! be found again, and is not kept.

use super::encode::identity_allowed;
use super::types::BondRecord;
use crate::error::BondError;
use nonos_bt_smp::pairing::Bond;
use nonos_bt_wire::addr::{AddrType, BdAddr};

/// This host pairs with a 16-octet key only (`Initiator::start`).
const KEY_SIZE: u8 = 16;

impl BondRecord {
    pub fn from_pairing(bond: &Bond, kind: AddrType, peer: &BdAddr) -> Result<Self, BondError> {
        let (identity_type, identity) = match bond.identity {
            Some(id) => id,
            None => (kind.wire(), peer.0),
        };
        if !identity_allowed(identity_type, &identity) {
            return Err(BondError::NoIdentity);
        }
        let record = Self {
            authenticated: bond.authenticated,
            identity_type,
            identity,
            ltk: bond.ltk,
            irk: bond.irk,
            key_size: KEY_SIZE,
        };
        record.check()?;
        Ok(record)
    }
}
