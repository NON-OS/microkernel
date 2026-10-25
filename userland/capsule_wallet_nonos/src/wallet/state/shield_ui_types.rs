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

//! What the shield service reports: holdings, the running job, a review
//! and a quote.

use alloc::string::String;

/* One coin's shielded holding, as the service wrote it. */
#[derive(Clone, Default)]
pub struct Held {
    pub spendable: String,
    pub pending: String,
}

/* The one job the service runs for this wallet, and when it was asked. */
#[derive(Clone, Copy)]
pub struct Waiting {
    pub op: u16,
    pub since_ms: i64,
}

/* A deposit, an approval or a self-settlement, checked and held by the
 * service under `id` until confirmed. */
#[derive(Clone, Default)]
pub struct Review {
    pub id: String,
    pub approval: bool,
    pub amount: String,
    pub pool_fee: String,
    pub shielded: String,
    pub max_network_fee: String,
    pub valid_for: String,
}

/* What a private payment or a withdrawal costs, read from the pool. */
#[derive(Clone, Default)]
pub struct Quote {
    pub network_fee: String,
    pub protocol_fee: String,
    pub total_fee: String,
}
