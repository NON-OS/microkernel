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

use serde::{Deserialize, Serialize};

use super::commerce::CommerceJson;
use super::release::ReleaseJson;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryJson {
    pub listing_id: String,
    pub capsule_id: String,
    pub name: String,
    pub publisher_name: String,
    pub publisher_pubkey: String,
    pub publisher_eth_address: String,
    pub description: String,
    /// The BLAKE3 of the listing's logo raster in hex, "" for none (schema 4).
    #[serde(default)]
    pub logo_hash: String,
    pub price: PriceJson,
    pub token: TokenJson,
    /// Required to encode: the encoder writes schema 4 only. Absent when a
    /// schema 2 blob is inspected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commerce: Option<CommerceJson>,
    pub releases: Vec<ReleaseJson>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceJson {
    pub kind: String,
    pub amount_atomic: String,
    pub period_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenJson {
    pub symbol: String,
    pub decimals: u8,
    pub chain_id: u64,
    #[serde(default)]
    pub contract_address: String,
}
