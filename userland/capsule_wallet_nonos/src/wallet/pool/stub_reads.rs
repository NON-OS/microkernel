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

//! The stub's simulation, revenue and quote answers: none is wired yet.

use super::decoded::{DecodedTx, StateDiff};
use super::quote_provider::QuoteProvider;
use super::revenue_provider::RevenueProvider;
use super::seam::Seam;
use super::sim_provider::SimProvider;
use super::stub::Stub;
use crate::wallet::swap::{Reserves, Token};

impl SimProvider for Stub {
    fn simulate(&self, _tx: &DecodedTx) -> Seam<StateDiff> {
        Seam::NotWired
    }
}

impl RevenueProvider for Stub {
    fn fee_bps(&self) -> Seam<u32> {
        Seam::NotWired
    }
    fn cumulative_revenue_wei(&self) -> Seam<[u8; 32]> {
        Seam::NotWired
    }
    fn staking_apr_bps(&self) -> Seam<u32> {
        Seam::NotWired
    }
    fn staked_wei(&self) -> Seam<[u8; 32]> {
        Seam::NotWired
    }
}

impl QuoteProvider for Stub {
    fn reserves(&self, _pay: Token, _receive: Token) -> Seam<Reserves> {
        Seam::NotWired
    }
    fn gas(&self, _pay: Token, _receive: Token) -> Seam<u64> {
        Seam::NotWired
    }
}
