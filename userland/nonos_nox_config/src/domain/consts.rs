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

//! The EIP-712 domains of the new contracts. Every one is version "1" and
//! carries chainId and verifyingContract, so a signature made for one chain or
//! one deployment recovers to a stranger anywhere else.

use crate::contract::Contract;
use crate::hex::hex32;

pub const EIP712_DOMAIN_TYPE: &str =
    "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
pub const EIP712_DOMAIN_TYPEHASH: [u8; 32] =
    hex32("8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f");

pub const VERSION: &str = "1";
pub const VERSION_HASH: [u8; 32] =
    hex32("c89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Domain {
    pub name: &'static str,
    /// keccak256 of `name`, as the domain separator hashes it.
    pub name_hash: [u8; 32],
    pub version: &'static str,
    pub contract: Contract,
}

pub const SETTLEMENT: Domain = Domain {
    name: "NONOS Settlement",
    name_hash: hex32("9fcd4fbd152ffd7f0627344c2cd9dcf7f407d40b5aaf3e3126b3b34452daccd2"),
    version: VERSION,
    contract: Contract::Settlement,
};

pub const ESCROW: Domain = Domain {
    name: "NONOS Escrow",
    name_hash: hex32("c100a8842b6497af2bf20fe8f9b9fc96d9bdf2b4cb4f4765ab31a68630f8dbf4"),
    version: VERSION,
    contract: Contract::Escrow,
};

pub const BOND: Domain = Domain {
    name: "NONOS Bond",
    name_hash: hex32("b4dfeee30953d2d8114d7740b1d8af1ff5a81505abd3efda74a42d5c4c3c6e0d"),
    version: VERSION,
    contract: Contract::PublisherBond,
};

pub const LISTING: Domain = Domain {
    name: "NONOS Listing",
    name_hash: hex32("a81009eb2257b4890b00597fd1ab29e72f614e8ef38d95d969b5d41ddc971669"),
    version: VERSION,
    contract: Contract::ListingRegistry,
};
