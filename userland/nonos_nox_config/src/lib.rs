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

//! Every chain constant the OS builds against: chain ids, contract addresses,
//! EIP-712 domains, type strings, typehashes and the tags inside derived ids.
//!
//! The owner redeploys the marketplace contracts from scratch, so nothing else
//! in the tree may name an address or a typehash. A redeploy is a change here
//! and nowhere else. A contract that is not deployed is `None`, and asking for
//! it is an error the caller must surface, never a zero address it might pay.

#![no_std]

mod chain;
mod contract;
mod deploy;
mod domain;
mod error;
mod hex;
mod lookup;
mod tags;
mod typed;
mod units;

pub use chain::{Chain, MAINNET, SEPOLIA};
pub use contract::Contract;
pub use deploy::{deployment, Deployment};
pub use domain::{domain_words, domain_words_at, Domain, BOND, EIP712_DOMAIN_TYPE};
pub use domain::{EIP712_DOMAIN_TYPEHASH, ESCROW, LISTING, SETTLEMENT, VERSION_HASH};
pub use error::ConfigError;
pub use hex::{hex20, hex32};
pub use lookup::address;
pub use tags::{BOND_TAG, CHANNEL_TAG, LISTING_TAG, ORDER_TAG, RELEASE_TAG, TAGS};
pub use typed::{TypedData, PUBLISHER_BINDING, RELEASE_ATTEST, TYPED, VOUCHER};
pub use units::{UNITS, UNIT_BYTE, UNIT_CALL, UNIT_INSTALL, UNIT_PROOF, UNIT_SECOND};
