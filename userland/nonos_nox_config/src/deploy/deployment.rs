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

use super::{mainnet, sepolia};
use crate::chain::Chain;
use crate::contract::Contract;

/// Where each contract lives on one chain. `None` until the owner deploys it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Deployment {
    pub nox_token: Option<[u8; 20]>,
    pub settlement: Option<[u8; 20]>,
    pub escrow: Option<[u8; 20]>,
    pub publisher_bond: Option<[u8; 20]>,
    pub listing_registry: Option<[u8; 20]>,
    pub proof_anchor: Option<[u8; 20]>,
}

impl Deployment {
    pub const fn get(&self, c: Contract) -> Option<[u8; 20]> {
        match c {
            Contract::NoxToken => self.nox_token,
            Contract::Settlement => self.settlement,
            Contract::Escrow => self.escrow,
            Contract::PublisherBond => self.publisher_bond,
            Contract::ListingRegistry => self.listing_registry,
            Contract::ProofAnchor => self.proof_anchor,
        }
    }
}

pub const fn deployment(chain: Chain) -> &'static Deployment {
    match chain {
        Chain::Mainnet => &mainnet::MAINNET,
        Chain::Sepolia => &sepolia::SEPOLIA,
    }
}
