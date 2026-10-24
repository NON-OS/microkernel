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

/*
 * What the publisher screen holds (B8, the plan): the path as typed, the
 * entry document read from it, whether the person confirmed its publisher
 * key, the last signature's line; the fields the Bond's and a release's
 * calls are typed into, the publisher transaction under way, the chain
 * views that grey its rows, and the settle request outstanding.
 */

use alloc::string::String;

use nonos_marketplace_abi::MarketplaceEntry;

use super::sending::{Asked, Sending};
use super::views::Views;
use crate::wallet::num::Amount;

/// The field typed into.
pub const FOCUS_PATH: u8 = 0;
pub const FOCUS_AMOUNT: u8 = 1;
pub const FOCUS_AGENT: u8 = 2;
pub const FOCUS_URI: u8 = 3;

#[derive(Default)]
pub struct PublisherUi {
    pub path: String,
    pub entry: Option<MarketplaceEntry>,
    pub confirmed: bool,
    pub news: Option<String>,
    pub focus: u8,
    /// The bond, in NOX as typed.
    pub amount: Amount,
    pub agent: String,
    pub uri: String,
    pub sending: Option<Sending>,
    pub views: Views,
    pub asked: Option<Asked>,
    /// The release whose attest is up for review, before it is signed.
    pub attest_ask: Option<usize>,
    /// The digest that review showed; Sign signs only while it is unchanged.
    pub attest_digest: Option<[u8; 32]>,
}
