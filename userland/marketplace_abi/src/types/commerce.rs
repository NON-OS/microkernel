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

//! What a schema 3 listing sells and to whom the money goes. The
//! operator signs it with the index; the publisher signs its hash in every
//! release, so neither can change the price or the payee behind the other.

extern crate alloc;

use alloc::vec::Vec;

/// How a purchase settles. Private funding is the wallet's choice, not this.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SettleMode {
    Free = 0,
    /// Pay per use, by meter vouchers.
    Voucher = 1,
    /// An encrypted package bought through an escrow order.
    Escrow = 2,
}

impl SettleMode {
    /// `None` for a byte this build does not know, which the decoder refuses:
    /// a mode it cannot read must never be read as free.
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            0 => Some(Self::Free),
            1 => Some(Self::Voucher),
            2 => Some(Self::Escrow),
            _ => None,
        }
    }
}

/// A proof a container must carry: the the plan kind and the program hash it
/// must verify under.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ProofReq {
    pub kind: u8,
    pub statement: [u8; 32],
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Commerce {
    pub payee: [u8; 20],
    pub bond_id: [u8; 32],
    /// keccak256 of the unit label, as `nonos_nox_config` names them.
    pub unit: [u8; 32],
    pub unit_price: u128,
    pub min_charge: u128,
    pub settle_mode: SettleMode,
    pub proof_reqs: Vec<ProofReq>,
    /// PublisherBinding, EIP-712, signed by `payee`: r, s, v.
    pub binding_sig: [u8; 65],
}
