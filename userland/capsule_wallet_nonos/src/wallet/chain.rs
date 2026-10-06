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

use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

pub struct Chain {
    pub id: u64,
    /// As the status line and the review say it.
    pub name: &'static str,
    /// The JSON-RPC hosts, reached over TLS 1.3 on port 443, in the order
    /// they are tried. Each serves a batch of every call a refresh makes at
    /// `/` without a key. One that fails is left for the next, and the next
    /// is asked its chain id before anything else is read from it.
    pub rpcs: &'static [&'static str],
    pub nox: [u8; 20],
    pub usdc: [u8; 20],
    /// The NOX staking contract, on mainnet only.
    pub staking: Option<[u8; 20]>,
    /// Whether the NOX Shield pool runs on this network.
    pub shield: bool,
    pub explorer: &'static str,
}

/// A hex digit's value. Any other byte indexes one past the end of a
/// one-element table, which stops the compiler on this line: a const fn
/// cannot return an error, and a sentinel would decode to a plausible byte.
const fn nibble(c: u8) -> u8 {
    let (value, bad) = match c {
        b'0'..=b'9' => (c - b'0', 0),
        b'a'..=b'f' => (c - b'a' + 10, 0),
        b'A'..=b'F' => (c - b'A' + 10, 0),
        _ => (0, 1),
    };
    let refuse = [0u8; 1];
    value | refuse[bad]
}

/// A 20-byte address from its 40 hex digits, checked when the crate builds.
pub const fn hex20(s: &str) -> [u8; 20] {
    let b = s.as_bytes();
    assert!(b.len() == 40);
    let mut out = [0u8; 20];
    let mut i = 0;
    while i < 20 {
        out[i] = (nibble(b[2 * i]) << 4) | nibble(b[2 * i + 1]);
        i += 1;
    }
    out
}

pub const MAINNET: Chain = Chain {
    id: 1,
    name: "Ethereum mainnet",
    rpcs: &["ethereum-rpc.publicnode.com", "mainnet.gateway.tenderly.co", "rpc.mevblocker.io"],
    nox: hex20("0a26c80be4e060e688d7c23addb92cbb5d2c9eca"),
    usdc: hex20("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
    staking: Some(hex20("a94d6009790ba13597a1e1b7cf4e1531ea513613")),
    shield: false,
    explorer: "etherscan.io",
};

pub const SEPOLIA: Chain = Chain {
    id: 11_155_111,
    name: "Sepolia",
    rpcs: &[
        "ethereum-sepolia-rpc.publicnode.com",
        "sepolia.gateway.tenderly.co",
        "rpc.sepolia.ethpandaops.io",
    ],
    nox: hex20("3e5249a65ca513d5e11260222e0d26f46b465d36"),
    usdc: hex20("1c7d4b196cb0c7b01d743fbc6116a902379c7238"),
    staking: None,
    shield: true,
    explorer: "sepolia.etherscan.io",
};

static PICKED: AtomicU8 = AtomicU8::new(0);

/* Which of each network's hosts is asked now, by its place in `rpcs`. */
static HOST: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)];

fn slot() -> &'static AtomicUsize {
    &HOST[usize::from(is_sepolia())]
}

/// The RPC host the picked network's requests go to now.
pub fn rpc_host() -> &'static str {
    let hosts = current().rpcs;
    hosts.get(slot().load(Ordering::Relaxed) % hosts.len().max(1)).copied().unwrap_or("")
}

/// `host` failed a request: the next one is asked from now on. Only when it
/// is still the one asked, so two requests that failed on it move one place.
/// True when the host changed.
pub fn rpc_failed(host: &str) -> bool {
    let hosts = current().rpcs;
    let at = slot().load(Ordering::Relaxed);
    if hosts.len() < 2 || hosts.get(at % hosts.len()) != Some(&host) {
        return false;
    }
    slot()
        .compare_exchange(at, (at + 1) % hosts.len(), Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
}

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
