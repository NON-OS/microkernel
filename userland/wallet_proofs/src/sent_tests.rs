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
 * The sent-payments ledger: a second payment never takes the first's
 * nonce, nothing is forgotten across a network switch, and each payment
 * ends confirmed, replaced or dropped from what the chain says, never
 * waiting forever.
 */

use crate::sent::{Fate, Ledger, Reading, Sent, CONFIRMATIONS, DROP_MS, REPLACED_AFTER};

const MAINNET: u64 = 1;
const SEPOLIA: u64 = 11_155_111;
const ME: [u8; 20] = [7; 20];
const ETH: u8 = 0;
const NOX: u8 = 1;

fn sent(chain: u64, nonce: u64, tag: u8) -> Sent {
    Sent {
        chain,
        from: ME,
        nonce,
        hash: [tag; 32],
        at_ms: 1_000,
        eth_cost: 100,
        token: Some((NOX, 5)),
        fate: Fate::Waiting,
        misses: 0,
        unknown: false,
    }
}

fn reading(receipt: Option<(u64, bool)>, head: u64, latest: u64, pending: u64) -> Reading {
    Reading { receipt, head, latest, pending }
}

#[path = "sent_fate_tests.rs"]
mod sent_fate_tests;
#[path = "sent_held_tests.rs"]
mod sent_held_tests;
#[path = "sent_nonce_tests.rs"]
mod sent_nonce_tests;
#[path = "sent_other_tests.rs"]
mod sent_other_tests;
