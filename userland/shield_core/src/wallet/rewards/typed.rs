//! The message a mainnet address signs, `Link(address mainnet,address testnet,uint256 nonce)`,
//! under the domain the registry fixes: its name, version 1 and chain id 1, with no contract, since
//! the mainnet wallet signs while it is connected to mainnet.

use crate::wallet::abi::word;
use sha3::{Digest, Keccak256};

pub(super) const NAME: &str = "NOX testnet rewards";
pub(super) const VERSION: &str = "1";
pub(super) const CHAIN_ID: u64 = 1;
const DOMAIN: &[u8] = b"EIP712Domain(string name,string version,uint256 chainId)";
const LINK: &[u8] = b"Link(address mainnet,address testnet,uint256 nonce)";

fn keccak(parts: &[&[u8]]) -> [u8; 32] {
    let mut hash = Keccak256::new();
    parts.iter().for_each(|part| hash.update(part));
    hash.finalize().into()
}

pub(super) fn address_word(address: &[u8; 20]) -> [u8; 32] {
    let mut out = [0u8; 32];
    out.split_at_mut(12).1.copy_from_slice(address);
    out
}

/// The domain separator, the same for every link.
pub(super) fn domain() -> [u8; 32] {
    let name = keccak(&[NAME.as_bytes()]);
    let version = keccak(&[VERSION.as_bytes()]);
    keccak(&[&keccak(&[DOMAIN]), &name, &version, &word(CHAIN_ID.into())])
}

/// The digest `mainnet` signs to link itself to `testnet` at its `nonce` in the registry.
pub fn digest(mainnet: &[u8; 20], testnet: &[u8; 20], nonce: u128) -> [u8; 32] {
    let link =
        keccak(&[&keccak(&[LINK]), &address_word(mainnet), &address_word(testnet), &word(nonce)]);
    keccak(&[b"\x19\x01", &domain(), &link])
}
