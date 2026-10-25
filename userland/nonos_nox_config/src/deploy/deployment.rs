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

use crate::contract::Contract;

/// One contract on one chain: its address, and the block it was deployed in,
/// where a log read starts. A block of 0 means not known, and settle refuses
/// log reads of that contract rather than page from genesis (v1.4.9).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Deployed {
    pub address: [u8; 20],
    pub block: u64,
}

pub const fn at(address: [u8; 20], block: u64) -> Option<Deployed> {
    Some(Deployed { address, block })
}

/// Where each contract lives on one chain. `None` until the owner deploys it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Deployment {
    pub nox_token: Option<Deployed>,
    pub settlement: Option<Deployed>,
    pub escrow: Option<Deployed>,
    pub publisher_bond: Option<Deployed>,
    pub listing_registry: Option<Deployed>,
    pub proof_anchor: Option<Deployed>,
    pub shield_pool: Option<Deployed>,
    pub amount_policy: Option<Deployed>,
}

impl Deployment {
    pub const fn deployed(&self, c: Contract) -> Option<Deployed> {
        match c {
            Contract::NoxToken => self.nox_token,
            Contract::Settlement => self.settlement,
            Contract::Escrow => self.escrow,
            Contract::PublisherBond => self.publisher_bond,
            Contract::ListingRegistry => self.listing_registry,
            Contract::ProofAnchor => self.proof_anchor,
            Contract::ShieldPool => self.shield_pool,
            Contract::AmountPolicy => self.amount_policy,
        }
    }

    pub const fn get(&self, c: Contract) -> Option<[u8; 20]> {
        match self.deployed(c) {
            Some(d) => Some(d.address),
            None => None,
        }
    }
}
