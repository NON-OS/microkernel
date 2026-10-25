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
mod escrow;
mod gas;
mod hex;
mod lookup;
mod offer;
mod shield_events;
mod tags;
mod typed;
mod units;

pub use chain::{Chain, MAINNET, MAINNET_BLOCK_MS, SEPOLIA, SEPOLIA_BLOCK_MS};
pub use contract::{Contract, CONTRACTS};
pub use deploy::{deployment, Deployed, Deployment};
pub use domain::{
    domain_answer_matches, domain_words, domain_words_at, Domain, BOND, EIP712_DOMAIN_TYPE,
};
pub use domain::{EIP712_DOMAIN_TYPEHASH, ESCROW, LISTING, SETTLEMENT, VERSION_HASH};
pub use error::ConfigError;
pub use escrow::{forwarder_initcode_hash, DELIVER_MARGIN, DISPUTE_WINDOW, MAX_DELIVERY};
pub use escrow::{ORDER_MIN_LEAD, UNBOND_DELAY_MIN};
pub use gas::{
    APPROVE_GAS, CLAIM_GAS, CLOSE_GAS, MAX_FEE_PER_GAS, MAX_PRIORITY_FEE_PER_GAS, OPEN_GAS,
};
pub use gas::{DISPUTE_GAS, REFUND_GAS, SETTLE_GAS_MAX, TOPUP_GAS};
pub use hex::{hex20, hex32};
pub use lookup::{address, deployed};
pub use offer::{ESCROW_POW_EFFORT, MAX_OFFERS, OFFER_TTL};
pub use shield_events::{INTENT_UNSHIELDED, NOTE_COMMITTED, PAYOUT_CLAIMED, PAYOUT_CREDITED};
pub use shield_events::{POLICY_PAUSED, POLICY_UNPAUSED, SHIELD_EVENTS};
pub use tags::{BOND_TAG, CHANNEL_TAG, LISTING_TAG, ORDER_TAG, REFUND_TAG, RELEASE_TAG, TAGS};
pub use typed::{TypedData, PUBLISHER_BINDING, RELEASE_ATTEST, TYPED, VOUCHER};
pub use units::{UNITS, UNIT_BYTE, UNIT_CALL, UNIT_INSTALL, UNIT_PROOF, UNIT_SECOND};
