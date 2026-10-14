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

//! The two networks this wallet keeps an account on: Ethereum mainnet and
//! Sepolia. One address serves both, since the key is the same; what differs
//! is where a request goes, which contracts the tokens live at, the chain id
//! every signature commits to, and whether the shield pool is there.
//!
//! Every request reads the network picked now, so a switch takes effect on
//! the next read, and every signature carries the picked chain id, so a
//! transaction reviewed on one network cannot be replayed on the other.

use core::sync::atomic::{AtomicU8, Ordering};

#[path = "chain_hex.rs"]
mod chain_hex;
#[path = "chain_host.rs"]
mod chain_host;
#[path = "chain_nets.rs"]
mod chain_nets;
pub use chain_host::{rpc_failed, rpc_host};
pub use chain_nets::{Chain, MAINNET, SEPOLIA};

static PICKED: AtomicU8 = AtomicU8::new(0);

pub fn current() -> &'static Chain {
    if is_sepolia() {
        &SEPOLIA
    } else {
        &MAINNET
    }
}

/// The name of the network a chain id is, as a review says it.
pub fn named(id: u64) -> &'static str {
    match id {
        1 => MAINNET.name,
        11_155_111 => SEPOLIA.name,
        _ => "an unknown network",
    }
}

pub fn is_sepolia() -> bool {
    PICKED.load(Ordering::Relaxed) == 1
}

/// Switch networks. Returns whether anything changed.
pub fn pick(sepolia: bool) -> bool {
    PICKED.swap(sepolia as u8, Ordering::Relaxed) != sepolia as u8
}

/// The `"result":"0x..."` an `eth_chainId` answer carries for this network.
pub fn chain_id_answer() -> alloc::string::String {
    alloc::format!("\"result\":\"0x{:x}\"", current().id)
}
