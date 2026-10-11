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

//! The schema 3 additions as a publisher writes them by hand. Empty hex
//! means zero, and an empty `bond_id` is derived from the payee and the
//! publisher key, which is the only value it may hold.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommerceJson {
    pub payee: String,
    #[serde(default)]
    pub bond_id: String,
    /// A unit label: "install", "proof", "call", "second" or "byte".
    pub unit: String,
    pub unit_price: String,
    #[serde(default)]
    pub min_charge: String,
    /// "free", "voucher" or "escrow".
    pub settle_mode: String,
    #[serde(default)]
    pub proof_reqs: Vec<ProofReqJson>,
    /// The payee's PublisherBinding signature, made in its wallet.
    #[serde(default)]
    pub binding_sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofReqJson {
    pub kind: u8,
    pub statement: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReleaseExtJson {
    #[serde(default)]
    pub commerce_hash: String,
    #[serde(default)]
    pub guest_policy: String,
    #[serde(default)]
    pub chunk_root: String,
    #[serde(default)]
    pub content_len: u64,
    #[serde(default)]
    pub proof_anchors: Vec<String>,
    /// The payee's ReleaseAttest signature, made in its wallet.
    #[serde(default)]
    pub release_attest_sig: String,
}
