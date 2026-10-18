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
//! Moving NOX found at a fresh address into the Shield (v1.4.56): the
//! review SHIELD_FROM_FRESH answers, and what its confirm sent. Standard
//! deposits only, after an approval of exactly their sum; what is below the
//! smallest standard size stays at the address, and the row says so. Pure.

use alloc::string::String;
use alloc::vec::Vec;

use shield_wire::{field, fields};

/// One move under way: the address, the NOX found there, and its last answer.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Moving {
    pub address: [u8; 20],
    /// The wei being moved, and whether it is ETH (paying its own gas) or NOX.
    pub amount: u128,
    pub eth: bool,
    pub review: Option<Reviewed>,
    /// Each transaction the last confirm sent, and why the rest did not go.
    pub sent: Vec<String>,
    pub stopped: Option<String>,
    /// When a Shield withdraw for this address's gas was started (unix s).
    pub gas_paid_s: Option<u64>,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Reviewed {
    pub id: u64,
    pub approval: bool,
    pub pieces: u32,
    pub shielded: String,
    pub max_network_fee: String,
    /// NOX wei left at the address, below the smallest standard size.
    pub remainder: u128,
    /// ETH wei the address lacks for the gas.
    pub short_eth: u128,
    pub refusal: Option<String>,
}

/// A review's answer; None unless its id and amounts read.
pub fn reviewed(body: &str) -> Option<Reviewed> {
    let n = |k: &str| field(body, k).and_then(|v| v.parse::<u128>().ok());
    Some(Reviewed {
        id: field(body, "id")?.parse().ok()?,
        approval: field(body, "approval") == Some("1"),
        pieces: field(body, "pieces")?.parse().ok()?,
        shielded: String::from(field(body, "shielded").unwrap_or("")),
        max_network_fee: String::from(field(body, "max_network_fee").unwrap_or("")),
        remainder: n("remainder")?,
        short_eth: n("short_eth")?,
        refusal: field(body, "refusal").map(String::from),
    })
}

/// A confirm's answer: each `tx` sent, at most `most`, and `stopped`.
pub fn sent(body: &str, most: usize) -> (Vec<String>, Option<String>) {
    let txs = fields(body, "tx").take(most).map(String::from).collect();
    (txs, field(body, "stopped").map(String::from))
}
