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

//! One release's whole verdict: the six gates and schema 3's three. The
//! listing and the readiness ops both answer with it, so they cannot differ.

use nonos_marketplace_abi::{CapsuleRelease, MarketplaceEntry};

use super::checks::evaluate;
use super::market::{escrow, market_gates};
use crate::chain_hosts::Host;

pub fn ready(
    signed_index: bool,
    entry: &MarketplaceEntry,
    rel: &CapsuleRelease,
    publisher_signed: bool,
) -> bool {
    evaluate(signed_index, &entry.listing_id, rel, publisher_signed, escrow(entry)).install_ready
        && market_gates(&Host, entry, rel, publisher_signed).all()
}
