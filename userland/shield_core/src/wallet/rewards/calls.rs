//! The calls of the registry: `link` and `unlink` from the testnet address, and the reads a screen
//! needs, pinned in the tests against the encoder of Foundry.

use super::typed::address_word;
use crate::wallet::abi::{bytes, word};

/// The registry on Sepolia, verified on Sourcify, deployed at block 11,815,577.
pub const REGISTRY: &str = "0xf1DC54d83b21D416ce619fA8C2E29C8381594225";
pub const REGISTRY_BYTES: [u8; 20] = [
    0xf1, 0xdc, 0x54, 0xd8, 0x3b, 0x21, 0xd4, 0x16, 0xce, 0x61, 0x9f, 0xa8, 0xc2, 0xe2, 0x9c, 0x83,
    0x81, 0x59, 0x42, 0x25,
];
/// Epoch 0 starts at midnight UTC on 1 October 2026.
pub const GENESIS: u64 = 1_790_812_800;
pub const EPOCH: u64 = 7 * 24 * 60 * 60;

const LINK: [u8; 4] = [0xc7, 0xd3, 0x50, 0xd1];
const UNLINK: [u8; 4] = [0x56, 0x5a, 0x8a, 0x3e];
pub(super) const NONCES: [u8; 4] = [0x7e, 0xce, 0xbe, 0x00];
pub(super) const MAINNET_OF: [u8; 4] = [0x7d, 0x9e, 0x02, 0x24];
pub(super) const LINK_OF: [u8; 4] = [0x52, 0x9a, 0x5e, 0x26];
pub(super) const CURRENT_EPOCH: [u8; 4] = [0x76, 0x67, 0x18, 0x08];

/// `link(mainnet, signature, contractWallet)`.
pub fn link_calldata(mainnet: &[u8; 20], signature: &[u8], contract_wallet: bool) -> Vec<u8> {
    let mut out = LINK.to_vec();
    out.extend_from_slice(&address_word(mainnet));
    out.extend_from_slice(&word(3 * 32));
    out.extend_from_slice(&word(u128::from(contract_wallet)));
    out.extend_from_slice(&bytes(signature));
    out
}

/// `unlink()`, which ends the link of the sender from the next epoch.
pub fn unlink_calldata() -> Vec<u8> {
    UNLINK.to_vec()
}

pub(super) fn of(selector: [u8; 4], address: &[u8; 20]) -> Vec<u8> {
    let mut out = selector.to_vec();
    out.extend_from_slice(&address_word(address));
    out
}
