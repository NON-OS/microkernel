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

//! The catalogue, as the market capsule serves it.

use alloc::vec::Vec;

use nonos_market_proto::{parse_list, OP_LIST_APPS};

use crate::store::listing::Listing;

use super::failure::Failure;
use super::wire::exchange;

/// Every listing, or why there are none to show.
pub fn fetch(port: u32, request_id: u32) -> Result<Vec<Listing>, Failure> {
    let body = exchange(port, OP_LIST_APPS, request_id, &[])?;
    let entries = parse_list(&body).ok_or(Failure::Malformed)?;
    Ok(entries.into_iter().map(|e| Listing::new(e.id, e.measurement, e.name, e.ready)).collect())
}
