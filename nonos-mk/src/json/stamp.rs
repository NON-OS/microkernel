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

//! A schema 3 release signs the commerce block it is sold under. Before the
//! publisher signs, its `commerce_hash` is set from the entry's block, so
//! price and payee are the publisher's word and not only the operator's.

use nonos_marketplace_abi::{commerce_hash_input, Blake3};

use crate::error::CliError;
use crate::hosts::Host;

use super::encode_commerce::to_commerce;
use super::hex_helpers::hex32;
use super::schema::EntryJson;

/// The hex `commerce_hash` every release of `entry` must carry.
pub(crate) fn commerce_hash_hex(entry: &EntryJson) -> Result<String, CliError> {
    let c = entry.commerce.as_ref().ok_or_else(|| {
        CliError::Json(format!("{}: no commerce block to sign under", entry.listing_id))
    })?;
    let publisher = hex32(&entry.publisher_pubkey, "publisher_pubkey")?;
    let commerce = to_commerce(c, &publisher)?;
    Ok(hex::encode(Host.blake3(&commerce_hash_input(&commerce))))
}
