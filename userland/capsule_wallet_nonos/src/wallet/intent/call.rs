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

/*
 * The publisher's transactions (plan/26), from the payee wallet: the
 * intent kinds a publisher sends, each with only the values the person
 * chose. The wallet builds the calldata here and never takes it from
 * settle or anyone else; settle's INTENT only checks and quotes it.
 */

use alloc::vec::Vec;

use nonos_nox_config::Contract;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Publisher {
    /// Kind 1: NOX approve, for exactly the flow's amount, to one of the
    /// three marketplace contracts.
    Approve { spender: Contract, amount: u128 },
    /// Kind 2: bond NOX under the listing's Ed25519 publisher key.
    Bond { publisher_key: [u8; 32], amount: u128 },
    /// Kind 3.
    SetDeliveryAgent { bond_id: [u8; 32], agent: [u8; 20] },
    /// Kind 4.
    RequestUnbond { bond_id: [u8; 32] },
    /// Kind 5, once UNBOND_DELAY has passed.
    WithdrawBond { bond_id: [u8; 32] },
    /// Kind 6: the registry's publish, its uri one settle would fetch.
    Publish {
        publisher_key: [u8; 32],
        app_id_hash: [u8; 32],
        release_id: [u8; 32],
        release_hash: [u8; 32],
        bond_id: [u8; 32],
        uri: Vec<u8>,
    },
    /// Kind 7: the registry's withdraw of one release.
    WithdrawRelease { listing_id: [u8; 32], release_id: [u8; 32] },
    /// Kind 8: anchor a proof container's root for one listing.
    Anchor { anchor: [u8; 32], kind: u8, listing_id: [u8; 32] },
    /// Kind 19: the Bond's withdrawCredit, to this account.
    WithdrawBondCredit { to: [u8; 20] },
}

impl Publisher {
    pub fn kind(&self) -> u8 {
        match self {
            Publisher::Approve { .. } => 1,
            Publisher::Bond { .. } => 2,
            Publisher::SetDeliveryAgent { .. } => 3,
            Publisher::RequestUnbond { .. } => 4,
            Publisher::WithdrawBond { .. } => 5,
            Publisher::Publish { .. } => 6,
            Publisher::WithdrawRelease { .. } => 7,
            Publisher::Anchor { .. } => 8,
            Publisher::WithdrawBondCredit { .. } => 19,
        }
    }
}
