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

//! A Linux listing's logo (OP_GET_LOGO), which the market serves only when
//! its bytes are the ones the signed index names. Other listings have none
//! to ask for.

use nonos_market_proto::logo::parse_logo;
pub use nonos_market_proto::logo::Logo;
use nonos_market_proto::{listing_body, OP_GET_LOGO};

use super::failure::Failure;
use super::service::next_id;
use super::wire::exchange;
use crate::store::listing::Source;

pub fn fetch(port: u32, source: Source, listing: &[u8]) -> Result<Option<Logo>, Failure> {
    if source != Source::Linux {
        return Ok(None);
    }
    let asked = listing_body(listing).ok_or(Failure::Malformed)?;
    parse_logo(&exchange(port, OP_GET_LOGO, next_id(), &asked)?).ok_or(Failure::Malformed)
}
