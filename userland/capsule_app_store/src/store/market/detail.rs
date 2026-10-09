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

//! Everything about one listing that the list reply leaves out.

use nonos_market_proto::{listing_body, parse_app, OP_GET_APP};

pub use nonos_market_proto::App as Detail;

use super::failure::Failure;
use super::wire::exchange;

pub fn fetch(port: u32, request_id: u32, listing: &[u8]) -> Result<Detail, Failure> {
    let body = exchange(port, OP_GET_APP, request_id, &listing_body(listing))?;
    parse_app(&body).ok_or(Failure::Malformed)
}
