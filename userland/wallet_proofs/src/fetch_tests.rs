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

//! The account refresh on each network: a batch reply of the shape the RPC
//! hosts answer parses into every field, and an answer is believed only
//! whole. The helpers here build those replies.

use crate::wallet::chain;
use crate::wallet::net::read_snapshot::{parse_snapshot, snapshot_request};

/* The network picked is one global, as in the capsule; a test holds it. */
static PICKED: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn hold() -> std::sync::MutexGuard<'static, ()> {
    PICKED.lock().unwrap_or_else(|e| e.into_inner())
}

const ADDR: [u8; 20] = [0xd8; 20];

fn quantity(id: u64, hex: &str) -> String {
    format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":\"0x{hex}\"}}")
}

fn word(n: u64) -> String {
    format!("{n:064x}")
}

const ETH: &str = "de0b6b3a7640000";
const NOX: &str = "3635c9adc5dea00000";
const USDC: &str = "5f5e100";

fn common() -> Vec<String> {
    vec![
        quantity(2, ETH),
        quantity(3, "7"),
        quantity(4, "3b9aca00"),
        quantity(10, NOX),
        quantity(15, USDC),
    ]
}

fn reply(parts: &[String]) -> Vec<u8> {
    format!("[{}]", parts.join(",")).into_bytes()
}

fn q(hex: &str) -> [u8; 32] {
    let v = u128::from_str_radix(hex, 16).unwrap();
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

#[path = "fetch_answer_tests.rs"]
mod fetch_answer_tests;
#[path = "fetch_reading_tests.rs"]
mod fetch_reading_tests;
#[path = "fetch_snapshot_tests.rs"]
mod fetch_snapshot_tests;
